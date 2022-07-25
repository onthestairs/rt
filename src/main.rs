use rand;
use std::rc::Rc;

use camera::Camera;
use colour::Colour;
use hittable::{Hittable, HittableList, Sphere};
use ray::Ray;
use v3::{random_in_unit_sphere, V3};

mod camera;
mod colour;
mod hittable;
mod image;
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
    world.add(Rc::new(Sphere::new(V3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(V3::new(0.0, -100.5, -1.0), 100.0)));

    // camera
    let camera = Camera::new();

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
        eprintln!("Finished pixel");
        return colour / (samples_per_pixel as f64);

        // color pixel_color = ray_color(r);
        // write_color(std::cout, pixel_color);
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

    if let Some(hit_record) = world.hit(ray, 0.0, f64::INFINITY) {
        let target = hit_record.point + hit_record.normal + random_in_unit_sphere();
        let new_ray = Ray::new(hit_record.point, target - hit_record.point);
        return 0.5 * ray_colour(&new_ray, world, depth - 1);
    }
    let unit_direction = v3::unit_vector(ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0);
}
