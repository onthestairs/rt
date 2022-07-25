use material::{Dielectric, Lambertian, Material, Metal};
use rand;
use std::sync::Arc;

use camera::Camera;
use colour::{random_colour, Colour};
use hittable::{Hittable, HittableList, Sphere};
use ray::Ray;
use v3::V3;

mod camera;
mod colour;
mod hittable;
mod image;
mod material;
mod ray;
mod v3;

fn scale(proportion: f64, min: f64, max: f64) -> f64 {
    return min + proportion * (max - min);
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Colour::new(0.5, 0.5, 0.5)));
    let ground_sphere = Sphere::new(V3::new(0.0, -1000.0, 0.0), 1000.0, ground_material);
    world.add(Arc::new(ground_sphere));

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
                    let albedo = random_colour() * random_colour();
                    Arc::new(Lambertian::new(albedo))
                } else if material_choice < 0.95 {
                    // metal
                    let albedo = random_colour() * random_colour();
                    let fuzz = scale(rand::random(), 0.5, 1.0);
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

    let material1 = Arc::new(Dielectric::new(1.5));
    let sphere1 = Sphere::new(V3::new(0.0, 1.0, 0.0), 1.0, material1);
    world.add(Arc::new(sphere1));

    let material2 = Arc::new(Lambertian::new(Colour::new(0.4, 0.2, 0.1)));
    let sphere2 = Sphere::new(V3::new(-4.0, 1.0, 0.0), 1.0, material2);
    world.add(Arc::new(sphere2));

    let material3 = Arc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    let sphere3 = Sphere::new(V3::new(4.0, 1.0, 0.0), 1.0, material3);
    world.add(Arc::new(sphere3));

    return world;
}

fn main() {
    // image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u64 = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as u64;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // world
    let world = random_scene();

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

    // render
    let i = image::generate_image(image_width, image_height, |row, col| {
        let mut colour = Colour::new(0.0, 0.0, 0.0);
        for _ in 0..samples_per_pixel {
            let u_d: f64 = rand::random();
            let v_d: f64 = rand::random();
            let u = (col as f64 + u_d) / (image_width as f64 - 1.0);
            let v = (row as f64 + v_d) / (image_height as f64 - 1.0);
            let ray = &camera.get_ray(u, v);
            colour = colour + ray_colour(&ray, &world, max_depth);
        }
        return colour.gamma_correct(1.0 / samples_per_pixel as f64);
    });
    image::print_image(image_width, image_height, i);
}

fn ray_colour<T>(ray: &Ray, world: &T, depth: u64) -> colour::Colour
where
    T: Hittable + Sync,
{
    if depth <= 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some((scattered_ray, attenuation)) = hit_record.material.scatter(ray, &hit_record) {
            return attenuation * ray_colour(&scattered_ray, world, depth - 1);
        } else {
            return Colour::new(0.0, 0.0, 0.0);
        }
    }
    let unit_direction = v3::unit_vector(ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0);
}
