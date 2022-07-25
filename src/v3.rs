use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy)]
pub struct V3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl V3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        return V3 { x, y, z };
    }

    pub fn length_squared(self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn length(self) -> f64 {
        return f64::sqrt(self.length_squared());
    }

    pub fn unit_vector(self) -> V3 {
        return self / self.length();
    }

    pub fn dot(a: V3, b: V3) -> f64 {
        return a.x * b.x + a.y * b.y + a.z * b.z;
    }

    pub fn near_zero(self) -> bool {
        let threshold = 1e-8;
        return self.x < threshold && self.y < threshold && self.x < threshold;
    }
}

impl Neg for V3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        return V3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        };
    }
}

impl Add for V3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for V3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<V3> for f64 {
    type Output = V3;

    fn mul(self, v: V3) -> Self::Output {
        return V3 {
            x: v.x * self,
            y: v.y * self,
            z: v.z * self,
        };
    }
}

impl Div<f64> for V3 {
    type Output = V3;

    fn div(self, s: f64) -> Self::Output {
        return V3 {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s,
        };
    }
}

pub fn unit_vector(v: V3) -> V3 {
    return v / v.length();
}

fn scale(proportion: f64, min: f64, max: f64) -> f64 {
    return min + proportion * (max - min);
}

pub fn random_vector(min: f64, max: f64) -> V3 {
    return V3 {
        x: scale(rand::random(), min, max),
        y: scale(rand::random(), min, max),
        z: rand::random(),
    };
}

pub fn random_in_unit_sphere() -> V3 {
    loop {
        let v = random_vector(-1.0, 1.0);
        if v.length_squared() < 1.0 {
            return v;
        }
    }
}

pub fn random_unit_vector() -> V3 {
    return unit_vector(random_in_unit_sphere());
}

pub fn reflect(v: V3, n: V3) -> V3 {
    return v - 2.0 * V3::dot(v, n) * n;
}
