#![allow(non_camel_case_types)]

use crate::linear::*;
use crate::numeric::*;

/// An invertible affine transform.
#[derive(Debug, Clone)]
pub struct Transform3<T: Num> {
    pub matrix: Matrix3<T>,
    pub origin: Vector3<T>,
}

pub type f32_xform3 = Transform3<f32>;
pub type f64_xform3 = Transform3<f64>;

pub type ff32_xform3 = Transform3<ff32>;

pub type r64_xform3 = Transform3<r64>;

impl<T: Num> Transform3<T> {
    #[inline(always)]
    pub fn new(matrix: Matrix3<T>, origin: Vector3<T>) -> Self {
        Self { matrix, origin }
    }

    #[inline(always)]
    pub fn map_point(&self, point: Vector3<T>) -> Vector3<T> {
        self.matrix * point + self.origin
    }

    #[inline(always)]
    pub fn map_vector(&self, vector: Vector3<T>) -> Vector3<T> {
        self.matrix * vector
    }

    pub fn invert(&self) -> Option<Self> {
        let matrix = self.matrix.inv()?;
        let translation = matrix * (-self.origin);

        Some(Self {
            matrix,
            origin: translation,
        })
    }

    /// First apply `self`, then apply `other`.
    pub fn chain(mut self, other: &Self) -> Self {
        self.origin = other.matrix * self.origin + other.origin;
        self.matrix = other.matrix * self.matrix;
        self
    }
}
