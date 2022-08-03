use std::sync::Arc;

use crate::{colour::Colour, v3::V3};

pub trait Texture {
    fn colour(&self, u: f64, v: f64, p: V3) -> Colour;
}

pub struct SolidColour {
    colour: Colour,
}

impl SolidColour {
    pub fn new(colour: Colour) -> Self {
        return SolidColour { colour };
    }
}

impl Texture for SolidColour {
    fn colour(&self, _u: f64, _v: f64, _p: V3) -> Colour {
        return self.colour;
    }
}

pub struct Checkers {
    scale: f64,
    odd: Arc<dyn Texture + Send + Sync>,
    even: Arc<dyn Texture + Send + Sync>,
}

impl Checkers {
    pub fn new<S: Texture + Send + Sync + 'static, T: Texture + Send + Sync + 'static>(
        scale: f64,
        even: S,
        odd: T,
    ) -> Self {
        return Checkers {
            scale,
            odd: Arc::new(even),
            even: Arc::new(odd),
        };
    }
    pub fn new_from_colours(scale: f64, even: Colour, odd: Colour) -> Self {
        return Checkers {
            scale,
            odd: Arc::new(SolidColour::new(odd)),
            even: Arc::new(SolidColour::new(even)),
        };
    }
}

impl Texture for Checkers {
    fn colour(&self, u: f64, v: f64, p: V3) -> Colour {
        let sines =
            f64::sin(self.scale * p.x) * f64::sin(self.scale * p.y) * f64::sin(self.scale * p.z);
        if sines < 0.0 {
            return self.odd.colour(u, v, p);
        } else {
            return self.even.colour(u, v, p);
        }
    }
}
