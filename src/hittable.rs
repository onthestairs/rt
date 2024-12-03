use std::sync::Arc;

use crate::{aabb::AABB, material::Material, ray::Ray, v3::V3};

#[derive(Clone)]
pub struct HitRecord {
    pub point: V3,
    pub normal: V3,
    pub time: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material + Sync + Send>,
}

impl HitRecord {
    pub fn new(
        time: f64,
        u: f64,
        v: f64,
        point: V3,
        outward_normal: V3,
        ray_direction: V3,
        material: Arc<dyn Material + Sync + Send>,
    ) -> Self {
        let front_face = V3::dot(ray_direction, outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        return HitRecord {
            time,
            u,
            v,
            point,
            normal,
            front_face,
            material,
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t_0: f64, t_1: f64) -> Option<AABB>;
}

pub struct HittableList {
    hittables: Vec<Arc<dyn Hittable + Sync + Send>>,
}

impl HittableList {
    pub fn new() -> Self {
        return HittableList { hittables: vec![] };
    }
    pub fn add(&mut self, hittable: Arc<dyn Hittable + Sync + Send>) {
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
                closest_t = hittable_record.time;
            }
        }
        return record;
    }

    fn bounding_box(&self, t_0: f64, t_1: f64) -> Option<AABB> {
        if self.hittables.len() == 0 {
            return None;
        }
        // let mut output_box = AABB::new(V3::new(0.0, 0.0, 0.0), V3::new(0.0, 0.0, 0.0));
        let mut output_box = self.hittables[0].bounding_box(t_0, t_1)?;
        for hittable in &self.hittables[1..] {
            if let Some(hittable_box) = hittable.bounding_box(t_0, t_1) {
                output_box = output_box + hittable_box;
            } else {
                return None;
            }
        }
        return Some(output_box);
    }
}

pub struct Sphere {
    centre: V3,
    radius: f64,
    material: Arc<dyn Material + Sync + Send>,
}

impl Sphere {
    pub fn new(centre: V3, radius: f64, material: Arc<dyn Material + Sync + Send>) -> Self {
        return Sphere {
            centre,
            radius,
            material,
        };
    }

    fn get_sphere_uv(&self, p: V3) -> (f64, f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = f64::acos(-p.y);
        let phi = f64::atan2(-p.z, p.x) + std::f64::consts::PI;

        let u = phi / (2.0 * std::f64::consts::PI);
        let v = theta / std::f64::consts::PI;
        return (u, v);
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
        let (u, v) = self.get_sphere_uv(point);
        let hit_record = HitRecord::new(
            t,
            u,
            v,
            point,
            outward_normal,
            ray.direction,
            self.material.clone(),
        );
        return Some(hit_record);
    }

    fn bounding_box(&self, t_0: f64, t_1: f64) -> Option<AABB> {
        let minimum = self.centre - V3::new(self.radius, self.radius, self.radius);
        let maximum = self.centre + V3::new(self.radius, self.radius, self.radius);
        let aabb = AABB::new(minimum, maximum);
        return Some(aabb);
    }
}

pub struct XYRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    /// The z-value for the rect
    k: f64,
    material: Arc<dyn Material + Send + Sync>,
}

impl XYRect {
    pub fn new(
        x0: f64,
        x1: f64,
        y0: f64,
        y1: f64,
        k: f64,
        material: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        return XYRect {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        };
    }

    fn get_rect_uv(&self, p: V3) -> (f64, f64) {
        todo!()
    }
}

impl Hittable for XYRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        // does the ray hit the z-plane within the range
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;
        // are the x and y coords within the rectangle
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let outward_normal = V3::new(0.0, 0.0, 1.0);
        let point = ray.at(t);
        let hit_record = HitRecord::new(
            t,
            u,
            v,
            point,
            outward_normal,
            ray.direction,
            self.material.clone(),
        );
        return Some(hit_record);
    }

    fn bounding_box(&self, _t_0: f64, _t_1: f64) -> Option<AABB> {
        let b = AABB::new(
            V3::new(self.x0, self.y0, self.k - 0.0001),
            V3::new(self.x1, self.y1, self.k + 0.0001),
        );
        return Some(b);
    }
}
