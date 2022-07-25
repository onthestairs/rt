use material::{Dielectric, Lambertian, Metal};
use rand;
use std::rc::Rc;

use camera::Camera;
use colour::Colour;
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

fn main() {
    // image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u64 = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u64;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // world
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.0)));
    let material_centre = Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Colour::new(0.8, 0.6, 0.2), 0.0));

    world.add(Rc::new(Sphere::new(
        V3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        V3::new(0.0, 0.0, -1.0),
        0.5,
        material_centre,
    )));
    world.add(Rc::new(Sphere::new(
        V3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        V3::new(-1.0, 0.0, -1.0),
        -0.45,
        material_left.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        V3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // camera
    let camera = Camera::new(
        V3::new(-2.0, 2.0, 1.0),
        V3::new(0.0, 0.0, -1.0),
        V3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
    );

    // render
    let i = image::generate_image(image_width, image_height, |row, col| {
        let mut colour = Colour::new(0.0, 0.0, 0.0);
        for _ in 0..samples_per_pixel {
            let u_d: f64 = rand::random();
            let v_d: f64 = rand::random();
            let u = (col as f64 + u_d) / (image_width as f64 - 1.0);
            let v = (row as f64 + v_d) / (image_height as f64 - 1.0);
            let ray = camera.get_ray(u, v);
            colour = colour + ray_colour(&ray, &world, max_depth);
        }
        return colour.gamma_correct(1.0 / samples_per_pixel as f64);
    });
    image::print_image(image_width, image_height, i);
}

fn ray_colour<T>(ray: &Ray, world: &T, depth: u64) -> colour::Colour
where
    T: Hittable,
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
