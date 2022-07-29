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
        // let accessors: Vec<Fn(V3) -> f64> = vec![|p: V3| p.x, |p: V3| p.y, |p: V3| p.z];
        let accessors = [0, 1, 2];
        return accessors.into_iter().all(|i| {
            let f = match i {
                0 => |p: V3| p.x,
                1 => |p: V3| p.y,
                2 => |p: V3| p.z,
                _ => unreachable!(),
            };
            let t0 = f64::min(
                (f(self.minimum) - f(ray.origin)) / f(ray.direction),
                (f(self.maximum) - f(ray.origin)) / f(ray.direction),
            );
            let t1 = f64::max(
                (f(self.minimum) - f(ray.origin)) / f(ray.direction),
                (f(self.maximum) - f(ray.origin)) / f(ray.direction),
            );
            let t_min = f64::max(t0, t_min);
            let t_max = f64::min(t1, t_max);
            return t_max > t_min;
        });
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
            f64::min(self.maximum.x, other.maximum.x),
            f64::min(self.maximum.y, other.maximum.y),
            f64::min(self.maximum.z, other.maximum.z),
        );
        return AABB::new(new_minimum, new_maximum);
    }
}
