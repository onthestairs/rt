use crate::{
    ray::Ray,
    v3::{random_in_unit_disk, unit_vector, V3},
};

pub struct Camera {
    origin: V3,
    lower_left_corner: V3,
    horizontal: V3,
    vertical: V3,
    u: V3,
    v: V3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: V3,
        look_at: V3,
        view_up: V3,
        vertical_field_of_view: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = vertical_field_of_view.to_radians();
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(look_from - look_at);
        let u = unit_vector(V3::cross(view_up, w));
        let v = V3::cross(w, u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        return Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        };
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let random_disk = self.lens_radius * random_in_unit_disk();
        let offset = random_disk.x * self.u + random_disk.y * self.v;
        let ray_direction =
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset;
        return Ray::new(self.origin + offset, ray_direction);
    }
}
