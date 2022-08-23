#![allow(non_camel_case_types)]

use super::*;

use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub type f32_3 = Vector3<f32>;
pub type f64_3 = Vector3<f64>;

pub type ff32_3 = Vector3<ff32>;

pub type r64_3 = Vector3<r64>;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector3<T: Num>(pub T, pub T, pub T);

impl<T: Num> Vector3<T> {
    #[inline(always)]
    pub fn new(x: T, y: T, z: T) -> Self {
        Self(x, y, z)
    }

    #[inline(always)]
    pub fn x(&self) -> T {
        self.0
    }

    #[inline(always)]
    pub fn y(&self) -> T {
        self.1
    }

    #[inline(always)]
    pub fn z(&self) -> T {
        self.2
    }
}

impl<T: Num> LinearSpace for Vector3<T> {
    type Scalar = T;
}

impl<T: Num> Display for Vector3<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.0, self.1, self.2)
    }
}

impl<T: Num> Neg for Vector3<T> {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl<T: Num> Add<Self> for Vector3<T> {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        self_from_3!(self.i + rhs.i)
    }
}

impl<T: Num> Sub<Self> for Vector3<T> {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        self_from_3!(self.i - rhs.i)
    }
}

impl<T: Num> AddAssign<Self> for Vector3<T> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        do_3!(self.i += rhs.i);
    }
}

impl<T: Num> SubAssign<Self> for Vector3<T> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        do_3!(self.i -= rhs.i);
    }
}

impl<T: Num> Mul<T> for Vector3<T> {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: T) -> Self {
        self_from_3!(self.i * rhs)
    }
}

impl<T: Num> Div<T> for Vector3<T> {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: T) -> Self {
        self_from_3!(self.i / rhs)
    }
}

impl<T: Num> MulAssign<T> for Vector3<T> {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: T) {
        do_3!(self.i *= rhs);
    }
}

impl<T: Num> DivAssign<T> for Vector3<T> {
    #[inline(always)]
    fn div_assign(&mut self, rhs: T) {
        do_3!(self.i /= rhs);
    }
}

impl<T: Num> Zero for Vector3<T> {
    const ZERO: Self = self_from_3!(T::ZERO);
}

impl<T: Num> One for Vector3<T> {
    const ONE: Self = self_from_3!(T::ONE);
}

impl<T: Num> Vector3<T> {
    pub const EX: Self = Self(T::ONE, T::ZERO, T::ZERO);
    pub const EY: Self = Self(T::ZERO, T::ONE, T::ZERO);
    pub const EZ: Self = Self(T::ZERO, T::ZERO, T::ONE);

    #[inline(always)]
    pub fn abs2(&self) -> T {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    #[inline(always)]
    pub fn abs(&self) -> T {
        self.abs2().sqrt()
    }

    #[inline(always)]
    pub fn norm(self) -> Self {
        self / self.abs()
    }

    #[inline(always)]
    pub fn dot(a: Self, b: Self) -> T {
        a.0 * b.0 + a.1 * b.1 + a.2 * b.2
    }

    #[inline(always)]
    pub fn cross(a: Self, b: Self) -> Self {
        Self(
            a.1 * b.2 - a.2 * b.1,
            a.2 * b.0 - a.0 * b.2,
            a.0 * b.1 - a.1 * b.0,
        )
    }

    /// a dot b cross c
    #[inline(always)]
    pub fn triple(a: Self, b: Self, c: Self) -> T {
        Self::dot(a, Self::cross(b, c))
    }

    #[inline(always)]
    pub fn min_coords(a: Self, b: Self) -> Self {
        Self(
            if a.0 < b.0 { a.0 } else { b.0 },
            if a.1 < b.1 { a.1 } else { b.1 },
            if a.2 < b.2 { a.2 } else { b.2 },
        )
    }

    #[inline(always)]
    pub fn max_coords(a: Self, b: Self) -> Self {
        Self(
            if a.0 > b.0 { a.0 } else { b.0 },
            if a.1 > b.1 { a.1 } else { b.1 },
            if a.2 > b.2 { a.2 } else { b.2 },
        )
    }
}

#[cfg(test)]
use quickcheck::Arbitrary;

#[cfg(test)]
impl Arbitrary for Vector3<r64> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        self_from_3!(r64::arbitrary(g))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use quickcheck_macros::quickcheck;

    LinearSpace_tests!(Vector3);

    #[quickcheck]
    fn dot_product(a: r64_3, b: r64_3, alpha: r64) -> bool {
        (r64_3::dot(a, b) == r64_3::dot(b, a))
            && (r64_3::dot(a, b * alpha) == alpha * r64_3::dot(a, b))
            && (r64_3::dot(a, r64_3::ZERO) == r64::ZERO)
    }

    #[quickcheck]
    fn cross_product(a: r64_3, b: r64_3) -> bool {
        r64_3::dot(r64_3::cross(a, b), a) == r64::ZERO
    }
}
