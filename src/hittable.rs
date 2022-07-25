use crate::{ray::Ray, v3::V3};

pub struct HitRecord {
    pub point: V3,
    pub normal: V3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    centre: V3,
    radius: f64,
}

impl Sphere {
    pub fn new(centre: V3, radius: f64) -> Self {
        return Sphere { centre, radius };
    }
}

impl Hittable for Sphere {
    fn hit(self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.centre;
        let a = ray.direction.length_squared();
        let half_b = V3::dot(oc, ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let mut t = (-half_b - sqrtd) / a;
        if t < t_min || t_max < t {
            t = (-half_b + sqrtd) / a;
            if t < t_min || t_max < t {
                return None;
            }
        }

        let point = ray.at(t);
        let normal = (point - self.centre) / self.radius;

        let hit_record = HitRecord { t, point, normal };
        return Some(hit_record);
    }
}
