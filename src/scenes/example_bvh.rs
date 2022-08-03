use std::sync::Arc;

use crate::{
    bvh::BVHNode,
    camera::Camera,
    colour::Colour,
    hittable::{Hittable, Sphere},
    material::{Dielectric, Lambertian, Material, Metal},
    texture::Checkers,
    utils::scale,
    v3::V3,
};

use super::Scene;

pub fn scene() -> Scene {
    let world = make_world();
    let aspect_ratio = 16.0 / 9.0;
    let camera = get_camera(aspect_ratio);
    return Scene {
        aspect_ratio,
        world: Box::new(world),
        camera,
    };
}

fn get_camera(aspect_ratio: f64) -> Camera {
    // camera
    let look_from = V3::new(13.0, 2.0, 3.0);
    let look_at = V3::new(0.0, 0.0, 0.0);
    let view_up = V3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
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

fn make_world() -> BVHNode {
    // let mut world = HittableList::new();
    let mut hittables: Vec<Arc<dyn Hittable + Send + Sync>> = vec![];

    // let ground_material = Arc::new(Lambertian::new(Colour::new(0.5, 0.5, 0.5)));
    let ground_material = Arc::new(Lambertian::new_from_texture(Checkers::new_from_colours(
        10.0,
        Colour::new(0.99, 0.45, 0.0),
        Colour::new(0.9, 0.9, 0.9),
    )));
    let ground_sphere = Sphere::new(V3::new(0.0, -1000.0, 0.0), 1000.0, ground_material);
    hittables.push(Arc::new(ground_sphere));

    for a in -11..11 {
        for b in -11..11 {
            let centre = V3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );
            let material_choice: f64 = rand::random();
            // point3 center(a + 0.9*random_double(), 0.2, b + 0.9*random_double());

            if (centre - V3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Arc<dyn Material + Send + Sync> = if material_choice < 0.8 {
                    // lambertian
                    let albedo = rand::random::<Colour>() * rand::random::<Colour>();
                    Arc::new(Lambertian::new(albedo))
                } else if material_choice < 0.95 {
                    // metal
                    let albedo = rand::random::<Colour>() * rand::random::<Colour>();
                    let fuzz = scale(rand::random(), 0.5, 1.0);
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Arc::new(Dielectric::new(1.5))
                };
                let sphere = Sphere::new(centre, 0.2, material);
                hittables.push(Arc::new(sphere));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    let sphere1 = Sphere::new(V3::new(0.0, 1.0, 0.0), 1.0, material1);
    hittables.push(Arc::new(sphere1));

    let material2 = Arc::new(Lambertian::new(Colour::new(0.4, 0.2, 0.1)));
    let sphere2 = Sphere::new(V3::new(-4.0, 1.0, 0.0), 1.0, material2);
    hittables.push(Arc::new(sphere2));

    let material3 = Arc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    let sphere3 = Sphere::new(V3::new(4.0, 1.0, 0.0), 1.0, material3);
    hittables.push(Arc::new(sphere3));

    eprintln!("Making BVHNode");
    let world = BVHNode::new(hittables, 0.0, f64::INFINITY);
    eprintln!("Finished making BVHNode");

    return world;
}
