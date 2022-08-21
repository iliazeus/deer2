#![allow(non_camel_case_types)]

use std::fmt::Debug;

pub trait Pixel: Debug + Clone + Copy {
    fn black() -> Self;
    fn white() -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct u8_rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel for u8_rgb {
    fn black() -> Self {
        Self {
            r: 0x00,
            g: 0x00,
            b: 0x00,
        }
    }

    fn white() -> Self {
        Self {
            r: 0xFF,
            g: 0xFF,
            b: 0xFF,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
}
