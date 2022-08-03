{
  description = "Ray tracing in one weekend";

  inputs = {
    # nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    nixpkgs.url = "nixpkgs/nixos-22.05";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        inherit (pkgs) lib;

        craneLib = crane.lib.${system};
        src = ./.;

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneLib.buildDepsOnly {
          buildInputs = [
            pkgs.libiconv
          ];
          inherit src;
        };

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        rt = craneLib.buildPackage {
          buildInputs = [
            pkgs.libiconv
          ];
          inherit cargoArtifacts src;
        };
      in
      {
        checks = {
          # Build the crate as part of `nix flake check` for convenience
          inherit rt;

          # Run clippy (and deny all warnings) on the crate source,
          # again, resuing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          rt-clippy = craneLib.cargoClippy {
            inherit cargoArtifacts src;
            cargoClippyExtraArgs = "-- --deny warnings";
          };

          # Check formatting
          rt-fmt = craneLib.cargoFmt {
            inherit src;
          };
        } // lib.optionalAttrs (system == "x86_64-linux") {
          # NB: cargo-tarpaulin only supports x86_64 systems
          # Check code coverage (note: this will not upload coverage anywhere)
          rt-coverage = craneLib.cargoTarpaulin {
            inherit cargoArtifacts src;
          };
        };

        packages.default = rt;

        apps.default = flake-utils.lib.mkApp {
          drv = rt;
        };

        apps.convert = flake-utils.lib.mkApp {
          drv = pkgs.writeShellScriptBin "convert-to-png" ''
            OTHER=''${1/ppm/png}
            echo $OTHER
            ${pkgs.imagemagick}/bin/convert $1 $OTHER
          '';
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks;

          buildInputs = [
            pkgs.libiconv
          ];
          nativeBuildInputs = with pkgs; [
            cargo
            rustc
          ];
        };
      });
}
