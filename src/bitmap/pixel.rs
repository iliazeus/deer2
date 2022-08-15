#![allow(non_camel_case_types)]

use std::fmt::Debug;
use std::num::Saturating;

pub trait Pixel: Debug + Clone {
    fn black() -> Self;
    fn white() -> Self;

    fn add(&self, other: &Self) -> Self;
}

#[derive(Debug, Clone, PartialEq)]
pub struct u8_rgb {
    pub r: Saturating<u8>,
    pub g: Saturating<u8>,
    pub b: Saturating<u8>,
}

impl u8_rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: Saturating(r),
            g: Saturating(g),
            b: Saturating(b),
        }
    }

    pub fn components(&self) -> (u8, u8, u8) {
        (self.r.0, self.g.0, self.b.0)
    }
}

impl Pixel for u8_rgb {
    fn black() -> Self {
        Self {
            r: Saturating(0x00),
            g: Saturating(0x00),
            b: Saturating(0x00),
        }
    }

    fn white() -> Self {
        Self {
            r: Saturating(0xFF),
            g: Saturating(0xFF),
            b: Saturating(0xFF),
        }
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct f32_rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Pixel for f32_rgb {
    fn black() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    fn white() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}
