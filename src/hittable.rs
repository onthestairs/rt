use std::rc::Rc;

use crate::{ray::Ray, v3::V3};

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub point: V3,
    pub normal: V3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(t: f64, point: V3, outward_normal: V3, ray_direction: V3) -> Self {
        let front_face = V3::dot(ray_direction, outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        return HitRecord {
            t,
            point,
            normal,
            front_face,
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    hittables: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        return HittableList { hittables: vec![] };
    }
    pub fn add(&mut self, hittable: Rc<dyn Hittable>) {
        self.hittables.push(hittable);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut record: Option<HitRecord> = None;
        let mut closest_t = t_max;
        for hittable in &self.hittables {
            if let Some(hittable_record) = hittable.hit(ray, t_min, closest_t) {
                record = Some(hittable_record.clone());
                closest_t = hittable_record.t;
            }
        }
        return record;
    }
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
        let outward_normal = (point - self.centre) / self.radius;
        let hit_record = HitRecord::new(t, point, outward_normal, ray.direction);
        return Some(hit_record);
    }
}
