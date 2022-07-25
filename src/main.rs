use std::num;

use colour::Colour;
use hittable::{Hittable, Sphere};
use ray::Ray;
use v3::V3;

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

    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = V3::new(0.0, 0.0, 0.0);
    let horizontal = V3::new(viewport_width, 0.0, 0.0);
    let vertical = V3::new(0.0, viewport_height, 0.0);
    let focal = V3::new(0.0, 0.0, focal_length);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focal;

    // render
    let i = image::generate_image(image_width, image_height, |row, col| {
        let u = col as f64 / (image_width as f64 - 1.0);
        let v = row as f64 / (image_height as f64 - 1.0);
        let ray_direction = lower_left_corner + u * horizontal + v * vertical - origin;
        let ray = Ray::new(origin, ray_direction);
        return ray_colour(ray);

        // color pixel_color = ray_color(r);
        // write_color(std::cout, pixel_color);
    });
    image::print_image(image_width, image_height, i);
}

/// Try to hit the given sphere with the ray
/// Return the `t` value where the ray first hits
fn hit_sphere(centre: V3, radius: f64, ray: &Ray) -> Option<f64> {
    let oc = ray.origin - centre;
    let a = V3::dot(ray.direction, ray.direction);
    let b = 2.0 * V3::dot(oc, ray.direction);
    let c = V3::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant > 0.0 {
        // your classic quadratic formula
        let t = (-b - f64::sqrt(discriminant)) / (2.0 * a);
        return Some(t);
    } else {
        return None;
    }
}

fn ray_colour(ray: Ray) -> colour::Colour {
    let sphere = Sphere::new(V3::new(0.0, 0.0, -1.0), 0.5);
    if let Some(hit_record) = sphere.hit(&ray, 0.0, 2000.0) {
        let unit_normal = hit_record.normal.unit_vector();
        return 0.5
            * Colour::new(
                unit_normal.x + 1.0,
                unit_normal.y + 1.0,
                unit_normal.z + 1.0,
            );
    }
    let unit_direction = v3::unit_vector(ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0);
}
