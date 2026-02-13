use std::ops::{Add, Div, Mul};

use rand::{Rng, rngs::SmallRng};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Colour {
    pub fn gamma_correct(&self) -> Self {
        Self {
            r: self.r.powf(1.0 / 2.2),
            g: self.g.powf(1.0 / 2.2),
            b: self.b.powf(1.0 / 2.2),
        }
    }

    pub fn as_array(&self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }

    fn clamp_to_u8(colour: f32) -> u8 {
        match colour {
            std::f32::MIN..=0.0 => 0,
            0.0..1.0 => (256.0 * colour).floor() as u8,
            1.0..std::f32::MAX => 255,
            _ => 0,
        }
    }

    pub fn to_u8(&self) -> [u8; 3] {
        self.as_array().map(|c| Self::clamp_to_u8(c))
    }

    pub fn black() -> Self {
        Colour {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn white() -> Self {
        Colour {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    pub fn random(rng: &mut SmallRng) -> Self {
        Colour {
            r: rng.random(),
            g: rng.random(),
            b: rng.random(),
        }
    }
}

impl Add for Colour {
    type Output = Colour;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Mul for Colour {
    type Output = Colour;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Mul<f32> for Colour {
    type Output = Colour;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: rhs * self.r,
            g: rhs * self.g,
            b: rhs * self.b,
        }
    }
}

impl Div<f32> for Colour {
    type Output = Colour;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl Mul<Colour> for f32 {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Self::Output {
        rhs * self
    }
}

impl From<[f32; 3]> for Colour {
    fn from(value: [f32; 3]) -> Self {
        Colour {
            r: value[0],
            g: value[1],
            b: value[2],
        }
    }
}
