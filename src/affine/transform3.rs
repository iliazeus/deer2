#![allow(non_camel_case_types)]

use crate::linear::*;
use crate::numeric::*;

/// An invertible affine transform.
pub struct Transform3<T: Num> {
    matrix: Matrix3<T>,
    translation: Vector3<T>,
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
    pub fn new(matrix: Matrix3<T>, translation: Vector3<T>) -> Option<Self> {
        if matrix.det() == T::zero() {
            None
        } else {
            Some(Self {
                matrix,
                translation,
            })
        }
    }

    pub fn transform_vec(&self, vec: Vector3<T>) -> Vector3<T> {
        &self.matrix * vec
    }

    pub fn transform_point(&self, point: Vector3<T>) -> Vector3<T> {
        &self.matrix * point + &self.translation
    }

    pub fn invert(&self) -> Self {
        let matrix = self.matrix.inv().unwrap();
        let translation = &matrix * (-self.translation.clone());

        Self {
            matrix,
            translation,
        }
    }
}
