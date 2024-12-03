use std::sync::Arc;

use crate::{
    camera::Camera,
    colour::Colour,
    hittable::{HittableList, Sphere, XYRect},
    material::{DiffuseLight, Lambertian, Metal},
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
    let look_from = V3::new(26.0, 3.0, 6.0);
    let look_at = V3::new(0.0, 2.0, 0.0);
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

fn make_world() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Colour::new(1.0, 0.0, 0.0)));
    let ground_sphere = Sphere::new(V3::new(0.0, -1000.0, 0.0), 1000.0, ground_material);
    world.add(Arc::new(ground_sphere));

    let sphere_material = Arc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    let sphere = Sphere::new(V3::new(0.0, 2.0, 0.0), 2.0, sphere_material);
    world.add(Arc::new(sphere));

    let light_material = Arc::new(DiffuseLight::new(Colour::new(4.0, 4.0, 4.0)));
    let light = XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, light_material);
    world.add(Arc::new(light));

    let light_material_2 = Arc::new(DiffuseLight::new(Colour::new(4.0, 4.0, 4.0)));
    let light_2 = Sphere::new(V3::new(0.0, 10.0, 0.0), 2.0, light_material_2);
    world.add(Arc::new(light_2));

    return world;
}
