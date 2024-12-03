use rand;

use colour::Colour;
use hittable::Hittable;
use ray::Ray;

mod aabb;
mod bvh;
mod camera;
mod colour;
mod hittable;
mod image;
mod material;
mod ray;
mod scenes;
mod texture;
mod utils;
mod v3;

enum Fidelity {
    Small,
    Full,
}

fn get_fidelity(fidelity: Fidelity, aspect_ratio: f64) -> (u64, u64, u64, u64) {
    let image_width: u64 = match fidelity {
        Fidelity::Small => 200,
        Fidelity::Full => 1200,
    };
    let image_height = (image_width as f64 / aspect_ratio) as u64;

    let samples_per_pixel = match fidelity {
        Fidelity::Small => 50,
        Fidelity::Full => 500,
    };
    let max_depth = match fidelity {
        Fidelity::Small => 10,
        Fidelity::Full => 50,
    };
    return (samples_per_pixel, max_depth, image_width, image_height);
}

fn main() {
    render_scene(scenes::SceneConfig::NTS, Fidelity::Full);
}

fn render_scene(scene: scenes::SceneConfig, fidelity: Fidelity) {
    let scene = scenes::get_scene(scene);
    let (samples_per_pixel, max_depth, image_width, image_height) =
        get_fidelity(fidelity, scene.aspect_ratio);

    // render
    let i = image::generate_image(image_width, image_height, |row, col| {
        let mut colour = Colour::new(0.0, 0.0, 0.0);
        for _ in 0..samples_per_pixel {
            let u_d: f64 = rand::random();
            let v_d: f64 = rand::random();
            let u = (col as f64 + u_d) / (image_width as f64 - 1.0);
            let v = (row as f64 + v_d) / (image_height as f64 - 1.0);
            let ray = &scene.camera.get_ray(u, v);
            colour = colour + ray_colour(&ray, &scene.world, max_depth);
        }
        return colour.gamma_correct(1.0 / samples_per_pixel as f64);
    });
    image::print_image(image_width, image_height, i);
}

fn ray_colour(ray: &Ray, world: &Box<dyn Hittable + Send + Sync>, depth: u64) -> colour::Colour
where
{
    if depth <= 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = world.hit(ray, 0.001, f64::INFINITY) {
        let emitted = hit_record
            .material
            .emitted(hit_record.u, hit_record.v, hit_record.point);
        if let Some((scattered_ray, attenuation)) = hit_record.material.scatter(ray, &hit_record) {
            return emitted + attenuation * ray_colour(&scattered_ray, world, depth - 1);
        } else {
            return emitted;
        }
    } else {
        return Colour::new(0.0, 0.0, 0.0);
        // let unit_direction = v3::unit_vector(ray.direction);
        // let t = 0.5 * (unit_direction.y + 1.0);
        // return (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0);
    }
}
