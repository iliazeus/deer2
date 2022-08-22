#![allow(non_camel_case_types)]

use super::*;

use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub type f32_2 = Vector2<f32>;
pub type f64_2 = Vector2<f64>;

pub type ff32_2 = Vector2<ff32>;

pub type r64_2 = Vector2<r64>;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector2<T: Num>(pub T, pub T);

impl<T: Num> Vector2<T> {
    #[inline(always)]
    pub fn new(x: T, y: T) -> Self {
        Self(x, y)
    }

    #[inline(always)]
    pub fn x(&self) -> T {
        self.0
    }

    #[inline(always)]
    pub fn y(&self) -> T {
        self.1
    }
}

impl<T: Num> LinearSpace for Vector2<T> {
    type Scalar = T;
}

impl<T: Num> Display for Vector2<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.0, self.1)
    }
}

impl<T: Num> Neg for Vector2<T> {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

impl<T: Num> Add<Self> for Vector2<T> {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        self_from_2!(self.i + rhs.i)
    }
}

impl<T: Num> Sub<Self> for Vector2<T> {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        self_from_2!(self.i - rhs.i)
    }
}

impl<T: Num> AddAssign<Self> for Vector2<T> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        do_2!(self.i += rhs.i);
    }
}

impl<T: Num> SubAssign<Self> for Vector2<T> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        do_2!(self.i -= rhs.i);
    }
}

impl<T: Num> Mul<T> for Vector2<T> {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: T) -> Self {
        self_from_2!(self.i * rhs)
    }
}

impl<T: Num> Div<T> for Vector2<T> {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: T) -> Self {
        self_from_2!(self.i / rhs)
    }
}

impl<T: Num> MulAssign<T> for Vector2<T> {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: T) {
        do_2!(self.i *= rhs);
    }
}

impl<T: Num> DivAssign<T> for Vector2<T> {
    #[inline(always)]
    fn div_assign(&mut self, rhs: T) {
        do_2!(self.i /= rhs);
    }
}

impl<T: Num> Zero for Vector2<T> {
    #[inline(always)]
    fn zero() -> Self {
        self_from_2!(T::zero())
    }
}

impl<T: Num> One for Vector2<T> {
    #[inline(always)]
    fn one() -> Self {
        self_from_2!(T::one())
    }
}

impl<T: Num> Vector2<T> {
    #[inline(always)]
    pub fn ex() -> Self {
        let _0 = T::zero();
        let _1 = T::one();
        Self(_1, _0)
    }

    #[inline(always)]
    pub fn ey() -> Self {
        let _0 = T::zero();
        let _1 = T::one();
        Self(_0, _1)
    }

    #[inline(always)]
    pub fn abs2(&self) -> T {
        self.0 * self.0 + self.1 * self.1
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
        a.0 * b.0 + a.1 * b.1
    }

    #[inline(always)]
    pub fn cross(a: Self, b: Self) -> T {
        a.0 * b.1 - a.1 * b.0
    }
}

#[cfg(test)]
use quickcheck::Arbitrary;

#[cfg(test)]
impl Arbitrary for Vector2<r64> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        self_from_2!(r64::arbitrary(g))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use quickcheck_macros::quickcheck;

    LinearSpace_tests!(Vector2);

    #[quickcheck]
    fn dot_product(a: r64_2, b: r64_2, alpha: r64) -> bool {
        (r64_2::dot(a, b) == r64_2::dot(b, a))
            && (r64_2::dot(a, b * alpha) == alpha * r64_2::dot(a, b))
            && (r64_2::dot(a, r64_2::zero()) == r64::zero())
    }
}
