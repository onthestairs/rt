use std::sync::Arc;

use rand::{prelude::StdRng, Rng, SeedableRng};

use crate::{
    camera::Camera,
    colour::Colour,
    hittable::{HittableList, Sphere, XYRect},
    material::{Dielectric, DiffuseLight, Lambertian, Material, Metal},
    texture::Checkers,
    utils::scale,
    v3::V3,
};

use super::Scene;

pub fn scene() -> Scene {
    let world = make_world(3, 2.0, true);
    let aspect_ratio = 16.0 / 16.0;
    let camera = get_camera(aspect_ratio);
    return Scene {
        aspect_ratio,
        world: Box::new(world),
        camera,
    };
}

fn get_camera(aspect_ratio: f64) -> Camera {
    // camera
    let look_from = V3::new(0.0, 3.0, 23.0);
    let look_at = V3::new(0.0, 0.0, 0.0);
    let view_up = V3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 23.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        view_up,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );
    return camera;
}

fn make_world(seed: u64, checkers_scale: f64, include_small_spheres: bool) -> HittableList {
    let mut rng = StdRng::seed_from_u64(seed);

    let mut world = HittableList::new();

    // let ground_material = Arc::new(Lambertian::new_from_texture(Checkers::new_from_colours(
    //     checkers_scale,
    //     Colour::new(0.2, 0.1, 0.9),
    //     Colour::new(0.9, 0.9, 0.9),
    // )));
    let ground_material = Arc::new(Lambertian::new(Colour::new(0.2, 0.1, 0.9)));
    let ground_sphere = Sphere::new(V3::new(0.0, -1000.0, 0.0), 1000.0, ground_material);
    world.add(Arc::new(ground_sphere));

    let orange = Colour::new(0.99, 0.2, 0.05);
    if include_small_spheres {
        for a in -11..11 {
            for b in -2..20 {
                let centre = V3::new(
                    a as f64 + 0.9 * rng.gen::<f64>(),
                    0.2,
                    b as f64 + 0.9 * rng.gen::<f64>(),
                );
                let material_choice: f64 = rng.gen();

                if (centre - V3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    let material: Arc<dyn Material + Send + Sync> = if material_choice < 0.3 {
                        // lambertian
                        let albedo = rng.gen::<Colour>() * rng.gen::<Colour>();
                        Arc::new(Lambertian::new(albedo))
                    } else if material_choice < 0.9 {
                        // metal
                        let albedo = orange;
                        let fuzz = scale(rng.gen(), 0.3, 0.6);
                        Arc::new(Metal::new(albedo, fuzz))
                    } else {
                        // glass
                        Arc::new(Dielectric::new(1.5))
                    };
                    let sphere = Sphere::new(centre, 0.2, material);
                    world.add(Arc::new(sphere));
                }
            }
        }
    }

    let material1 = Arc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    let sphere1 = Sphere::new(V3::new(0.0, 1.0, 0.0), 1.0, material1);
    world.add(Arc::new(sphere1));

    let material2 = Arc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    let sphere2 = Sphere::new(V3::new(-2.5, 1.0, 2.0), 1.0, material2);
    world.add(Arc::new(sphere2));

    let material3 = Arc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    let sphere3 = Sphere::new(V3::new(2.5, 1.0, 0.0), 1.0, material3);
    world.add(Arc::new(sphere3));

    // lights
    let light_strength = 10.0;
    let light_material = Arc::new(DiffuseLight::new(Colour::new(
        light_strength,
        light_strength,
        light_strength,
    )));
    let light = Sphere::new(V3::new(0.0, 10.0, 0.0), 2.0, light_material);
    world.add(Arc::new(light));

    return world;
}
