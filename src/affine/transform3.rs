#![allow(non_camel_case_types)]

use crate::linear::*;
use crate::numeric::*;

/// An invertible affine transform.
#[derive(Debug, Clone)]
pub struct Transform3<T: Num> {
    matrix: Matrix3<T>,
    origin: Vector3<T>,
}

pub type i8_xform3 = Transform3<i8>;
pub type i16_xform3 = Transform3<i16>;
pub type i32_xform3 = Transform3<i32>;
pub type i64_xform3 = Transform3<i64>;

pub type isize_xform3 = Transform3<isize>;

pub type f32_xform3 = Transform3<f32>;
pub type f64_xform3 = Transform3<f64>;

pub type r64_xform3 = Transform3<r64>;

impl<T: Num> Transform3<T> {
    #[inline(always)]
    pub fn new(matrix: Matrix3<T>, origin: Vector3<T>) -> Self {
        Self { matrix, origin }
    }

    #[inline(always)]
    pub fn map_point(&self, point: Vector3<T>) -> Vector3<T> {
        &self.matrix * point + &self.origin
    }

    #[inline(always)]
    pub fn map_vector(&self, vector: Vector3<T>) -> Vector3<T> {
        &self.matrix * vector
    }

    pub fn invert(&self) -> Option<Self> {
        let matrix = self.matrix.inv()?;
        let translation = &matrix * (-self.origin.clone());

        Some(Self {
            matrix,
            origin: translation,
        })
    }
}
