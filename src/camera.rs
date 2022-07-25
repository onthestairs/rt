use crate::{
    ray::Ray,
    v3::{unit_vector, V3},
};

pub struct Camera {
    origin: V3,
    lower_left_corner: V3,
    horizontal: V3,
    vertical: V3,
}

impl Camera {
    pub fn new(
        look_from: V3,
        look_at: V3,
        view_up: V3,
        vertical_field_of_view: f64,
        aspect_ratio: f64,
    ) -> Camera {
        let theta = vertical_field_of_view.to_radians();
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(look_from - look_at);
        let u = unit_vector(V3::cross(view_up, w));
        let v = V3::cross(w, u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        return Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        };
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let ray_direction =
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin;
        return Ray::new(self.origin, ray_direction);
    }
}
