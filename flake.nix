{
  description = "csolve";

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
          inherit src;
          buildInputs = [
            pkgs.libiconv
            pkgs.postgresql
            pkgs.openssl.dev
            pkgs.openssl
            pkgs.darwin.apple_sdk.frameworks.Security
            pkgs.darwin.apple_sdk.frameworks.CoreServices
            pkgs.darwin.apple_sdk.frameworks.CoreFoundation
          ];
          OPENSSL_DIR = "${pkgs.openssl.dev}";
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
        };

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        csolve = craneLib.buildPackage {
          inherit cargoArtifacts src;
          buildInputs = [
            pkgs.libiconv
            pkgs.postgresql
            pkgs.openssl.dev
            pkgs.darwin.apple_sdk.frameworks.Security
            pkgs.darwin.apple_sdk.frameworks.CoreServices
            pkgs.darwin.apple_sdk.frameworks.CoreFoundation
          ];
          OPENSSL_DIR = "${pkgs.openssl.dev}";
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
          doCheck = false;
        };
      in
      {
        checks = {
          # Build the crate as part of `nix flake check` for convenience
          inherit csolve;

          # Run clippy (and deny all warnings) on the crate source,
          # again, resuing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          csolve-clippy = craneLib.cargoClippy {
            inherit cargoArtifacts src;
            cargoClippyExtraArgs = "-- --deny warnings";
          };

          # Check formatting
          csolve-fmt = craneLib.cargoFmt {
            inherit src;
          };
        } // lib.optionalAttrs (system == "x86_64-linux") {
          # NB: cargo-tarpaulin only supports x86_64 systems
          # Check code coverage (note: this will not upload coverage anywhere)
          csolve-coverage = craneLib.cargoTarpaulin {
            inherit cargoArtifacts src;
          };
        };

        packages.default = csolve;

        apps.default = flake-utils.lib.mkApp {
          drv = csolve;
        };

        packages.container = pkgs.dockerTools.buildLayeredImage {
          name = "csolve";
          tag = csolve.version;
          created = "now";
          contents = csolve;
          config.Cmd = [ "${csolve}/bin/server" ];
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks;

          buildInputs = [
            pkgs.libiconv
            pkgs.postgresql
            pkgs.openssl.dev
            pkgs.darwin.apple_sdk.frameworks.Security
            pkgs.darwin.apple_sdk.frameworks.CoreServices
            pkgs.darwin.apple_sdk.frameworks.CoreFoundation
          ];
          nativeBuildInputs = with pkgs; [
            cargo
            rustc
          ];
          OPENSSL_DIR = "${pkgs.openssl.dev}";
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";

          CSOLVE_STORE_TYPE = "file_system";
          CSOLVE_STORE_DIR = "./data";
          DATABASE_URL = "postgres://csolve:B9SmU3WseeuzY9A@localhost:5432/csolve";
        };
      });
}
