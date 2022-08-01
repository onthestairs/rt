use std::ops::Add;

use crate::{ray::Ray, v3::V3};

#[derive(Clone, Copy)]
pub struct AABB {
    pub minimum: V3,
    pub maximum: V3,
}

impl AABB {
    pub fn new(minimum: V3, maximum: V3) -> Self {
        return AABB { minimum, maximum };
    }
    pub fn does_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..2 {
            let inv_d = 1.0 / ray.direction.get_by_index(a);
            let mut t0 = (self.minimum.get_by_index(a) - ray.origin.get_by_index(a)) * inv_d;
            let mut t1 = (self.minimum.get_by_index(a) - ray.origin.get_by_index(a)) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        return true;
    }
}

impl Add for AABB {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let new_minimum = V3::new(
            f64::min(self.minimum.x, other.minimum.x),
            f64::min(self.minimum.y, other.minimum.y),
            f64::min(self.minimum.z, other.minimum.z),
        );
        let new_maximum = V3::new(
            f64::max(self.maximum.x, other.maximum.x),
            f64::max(self.maximum.y, other.maximum.y),
            f64::max(self.maximum.z, other.maximum.z),
        );
        return AABB::new(new_minimum, new_maximum);
    }
}
