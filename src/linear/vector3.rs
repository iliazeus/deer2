#![allow(non_camel_case_types)]

use crate::numeric::*;

use super::*;

use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub type i8_3 = Vector3<i8>;
pub type i16_3 = Vector3<i16>;
pub type i32_3 = Vector3<i32>;
pub type i64_3 = Vector3<i64>;

pub type isize_3 = Vector3<isize>;

pub type f32_3 = Vector3<f32>;
pub type f64_3 = Vector3<f64>;

pub type r64_3 = Vector3<r64>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Vector3<T: Num>(pub T, pub T, pub T);

impl<T: Num> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self(x, y, z)
    }
}

impl<T: Num> LinearSpace for Vector3<T> {
    type Scalar = T;
}

macro_rules! do_3 {
    ($lhs:ident.i $op:tt $rhs:ident) => {
        $lhs.0 $op $rhs;
        $lhs.1 $op $rhs;
        $lhs.2 $op $rhs;
   };

    ($lhs:ident.i $op:tt $rhs:ident.i) => {
        $lhs.0 $op $rhs.0;
        $lhs.1 $op $rhs.1;
        $lhs.2 $op $rhs.2;
    };
}

macro_rules! self_from_3 {
    ($arg:expr) => {
        Self($arg, $arg, $arg)
    };
}

impl<T: Num> Display for Vector3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.0, self.1, self.2)
    }
}

impl<T: Num> Neg for Vector3<T> {
    type Output = Self;
    #[inline(always)]
    fn neg(mut self) -> Self::Output {
        self.0 = -self.0;
        self.1 = -self.1;
        self.2 = -self.2;
        self
    }
}

impl<T: Num> Add<&Self> for Vector3<T> {
    type Output = Self;
    #[inline(always)]
    fn add(mut self, rhs: &Self) -> Self {
        do_3!(self.i += rhs.i);
        self
    }
}

impl<T: Num> Sub<&Self> for Vector3<T> {
    type Output = Self;
    #[inline(always)]
    fn sub(mut self, rhs: &Self) -> Self {
        do_3!(self.i -= rhs.i);
        self
    }
}

impl<T: Num> AddAssign<&Self> for Vector3<T> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: &Self) {
        do_3!(self.i += rhs.i);
    }
}

impl<T: Num> SubAssign<&Self> for Vector3<T> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: &Self) {
        do_3!(self.i -= rhs.i);
    }
}

impl<T: Num> Mul<T> for Vector3<T> {
    type Output = Self;
    #[inline(always)]
    fn mul(mut self, rhs: T) -> Self {
        do_3!(self.i *= rhs);
        self
    }
}

impl<T: Num> Div<T> for Vector3<T> {
    type Output = Self;
    #[inline(always)]
    fn div(mut self, rhs: T) -> Self {
        do_3!(self.i /= rhs);
        self
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
    #[inline(always)]
    fn zero() -> Self {
        self_from_3!(T::zero())
    }
}

impl<T: Num> One for Vector3<T> {
    #[inline(always)]
    fn one() -> Self {
        self_from_3!(T::one())
    }
}

impl<T: Num> Vector3<T> {
    #[inline(always)]
    pub fn e0() -> Self {
        let _0 = T::zero();
        let _1 = T::one();
        Self(_1, _0, _0)
    }

    #[inline(always)]
    pub fn e1() -> Self {
        let _0 = T::zero();
        let _1 = T::one();
        Self(_0, _1, _0)
    }

    #[inline(always)]
    pub fn e2() -> Self {
        let _0 = T::zero();
        let _1 = T::one();
        Self(_0, _0, _1)
    }

    #[inline(always)]
    pub fn abs2(&self) -> T {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    #[inline(always)]
    pub fn dot(a: &Self, b: &Self) -> T {
        a.0 * b.0 + a.1 * b.1 + a.2 * b.2
    }

    #[inline(always)]
    pub fn cross(a: &Self, b: &Self) -> Self {
        Self(
            a.1 * b.2 - a.2 * b.1,
            a.2 * b.0 - a.0 * b.2,
            a.0 * b.1 - a.1 * b.0,
        )
    }
}

impl Vector3<f32> {
    #[inline(always)]
    pub fn abs(&self) -> f32 {
        self.abs2().sqrt()
    }
}

impl Vector3<f64> {
    #[inline(always)]
    pub fn abs(&self) -> f64 {
        self.abs2().sqrt()
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
        (r64_3::dot(&a, &b) == r64_3::dot(&b, &a))
            && (r64_3::dot(&a, &(b.clone() * alpha)) == alpha * r64_3::dot(&a, &b))
            && (r64_3::dot(&a, &r64_3::zero()) == r64::zero())
    }

    #[quickcheck]
    fn cross_product(a: r64_3, b: r64_3) -> bool {
        r64_3::dot(&r64_3::cross(&a, &b), &a) == r64::zero()
    }
}
