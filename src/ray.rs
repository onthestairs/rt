use crate::v3::V3;

pub struct Ray {
    pub origin: V3,
    pub direction: V3,
}

impl Ray {
    pub fn new(origin: V3, direction: V3) -> Ray {
        return Ray { origin, direction };
    }

    pub fn at(&self, t: f64) -> V3 {
        return self.origin + t * self.direction;
    }
}
