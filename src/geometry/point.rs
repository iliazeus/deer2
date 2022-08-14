use crate::linear::*;
use crate::numeric::*;

use super::*;

pub trait Point: Geometry {
    fn from_coords(coords: Vector3<Self::Num>) -> Self;
    fn into_coords(self) -> Vector3<Self::Num>;

    fn coords(&self) -> Vector3<Self::Num>;
}

impl<T: Num> Point for Vector3<T> {
    #[inline(always)]
    fn from_coords(coords: Vector3<Self::Num>) -> Self {
        coords
    }

    #[inline(always)]
    fn into_coords(self) -> Vector3<Self::Num> {
        self
    }

    #[inline(always)]
    fn coords(&self) -> Vector3<Self::Num> {
        self.clone()
    }
}
