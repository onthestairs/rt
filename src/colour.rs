use std::ops::{Add, Div, Mul};

pub struct Colour {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Colour {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        return Colour { red, green, blue };
    }
}

impl Add for Colour {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Mul<Colour> for f64 {
    type Output = Colour;

    fn mul(self, c: Colour) -> Self::Output {
        return Colour {
            red: c.red * self,
            green: c.green * self,
            blue: c.blue * self,
        };
    }
}

impl Div<f64> for Colour {
    type Output = Colour;

    fn div(self, s: f64) -> Self::Output {
        return Colour {
            red: self.red / s,
            green: self.green / s,
            blue: self.blue / s,
        };
    }
}
