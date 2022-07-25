use crate::{ray::Ray, v3::V3};

pub struct Camera {
    origin: V3,
    lower_left_corner: V3,
    horizontal: V3,
    vertical: V3,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio: f64 = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = V3::new(0.0, 0.0, 0.0);
        let horizontal = V3::new(viewport_width, 0.0, 0.0);
        let vertical = V3::new(0.0, viewport_height, 0.0);
        let focal = V3::new(0.0, 0.0, focal_length);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focal;
        return Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        };
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let ray_direction =
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;
        return Ray::new(self.origin, ray_direction);
    }
}
