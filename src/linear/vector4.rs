#![allow(non_camel_case_types)]

use crate::numeric::*;

use super::*;

use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub type u8_4 = Vector4<u8>;
pub type u16_4 = Vector4<u16>;
pub type u32_4 = Vector4<u32>;
pub type u64_4 = Vector4<u64>;

pub type i8_4 = Vector4<i8>;
pub type i16_4 = Vector4<i16>;
pub type i32_4 = Vector4<i32>;
pub type i64_4 = Vector4<i64>;

pub type usize_4 = Vector4<usize>;
pub type isize_4 = Vector4<isize>;

pub type f32_4 = Vector4<f32>;
pub type f64_4 = Vector4<f64>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Vector4<T: Num>(T, T, T, T);

impl<T: Num> LinearSpace for Vector4<T> {
    type Scalar = T;
}

macro_rules! do_4 {
    ($lhs:ident.i $op:tt $rhs:ident) => {
        $lhs.0 $op $rhs;
        $lhs.1 $op $rhs;
        $lhs.2 $op $rhs;
        $lhs.3 $op $rhs;
    };

    ($lhs:ident.i $op:tt $rhs:ident.i) => {
        $lhs.0 $op $rhs.0;
        $lhs.1 $op $rhs.1;
        $lhs.2 $op $rhs.2;
        $lhs.3 $op $rhs.3;
    };
}

macro_rules! self_from_4 {
    ($arg:expr) => {
        Self($arg, $arg, $arg, $arg)
    };
}

impl<T: Num> Display for Vector4<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.0, self.1, self.2, self.3)
    }
}

impl<T: Num> Add<&Self> for Vector4<T> {
    type Output = Self;
    #[inline(always)]
    fn add(mut self, rhs: &Self) -> Self {
        do_4!(self.i += rhs.i);
        self
    }
}

impl<T: Num> Sub<&Self> for Vector4<T> {
    type Output = Self;
    #[inline(always)]
    fn sub(mut self, rhs: &Self) -> Self {
        do_4!(self.i -= rhs.i);
        self
    }
}

impl<T: Num> AddAssign<&Self> for Vector4<T> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: &Self) {
        do_4!(self.i += rhs.i);
    }
}

impl<T: Num> SubAssign<&Self> for Vector4<T> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: &Self) {
        do_4!(self.i -= rhs.i);
    }
}

impl<T: Num> Mul<T> for Vector4<T> {
    type Output = Self;
    #[inline(always)]
    fn mul(mut self, rhs: T) -> Self {
        do_4!(self.i *= rhs);
        self
    }
}

impl<T: Num> Div<T> for Vector4<T> {
    type Output = Self;
    #[inline(always)]
    fn div(mut self, rhs: T) -> Self {
        do_4!(self.i /= rhs);
        self
    }
}

impl<T: Num> MulAssign<T> for Vector4<T> {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: T) {
        do_4!(self.i *= rhs);
    }
}

impl<T: Num> DivAssign<T> for Vector4<T> {
    #[inline(always)]
    fn div_assign(&mut self, rhs: T) {
        do_4!(self.i /= rhs);
    }
}

impl<T: Num> Zero for Vector4<T> {
    #[inline(always)]
    fn zero() -> Self {
        self_from_4!(T::zero())
    }
}

impl<T: Num> One for Vector4<T> {
    #[inline(always)]
    fn one() -> Self {
        self_from_4!(T::one())
    }
}

impl<T: Num> Vector4<T> {
    #[inline(always)]
    pub fn e0() -> Self {
        let _0 = T::zero();
        let _1 = T::one();
        Self(_1, _0, _0, _0)
    }

    #[inline(always)]
    pub fn e1() -> Self {
        let _0 = T::zero();
        let _1 = T::one();
        Self(_0, _1, _0, _0)
    }

    #[inline(always)]
    pub fn e2() -> Self {
        let _0 = T::zero();
        let _1 = T::one();
        Self(_0, _0, _1, _0)
    }

    #[inline(always)]
    pub fn e3() -> Self {
        let _0 = T::zero();
        let _1 = T::one();
        Self(_0, _0, _0, _1)
    }

    #[inline(always)]
    pub fn abs2(&self) -> T {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2 + self.3 * self.3
    }

    #[inline(always)]
    pub fn dot(a: &Self, b: &Self) -> T {
        a.0 * b.0 + a.1 * b.1 + a.2 * b.2 + a.3 * b.3
    }

    #[inline(always)]
    pub fn cross(a: &Self, b: &Self) -> Self {
        Self(
            a.1 * b.2 - a.2 * b.1,
            a.2 * b.0 - a.0 * b.2,
            a.0 * b.1 - a.1 * b.0,
            a.3 * b.3,
        )
    }
}

impl Vector4<f32> {
    #[inline(always)]
    pub fn abs(&self) -> f32 {
        self.abs2().sqrt()
    }
}

impl Vector4<f64> {
    #[inline(always)]
    pub fn abs(&self) -> f64 {
        self.abs2().sqrt()
    }
}

#[cfg(test)]
use quickcheck::Arbitrary;

#[cfg(test)]
impl Arbitrary for Vector4<i32> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        // we limit the range to protect from overflow
        self_from_4!(Small::arbitrary(g).0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use quickcheck_macros::quickcheck;

    LinearSpace_tests!(i32_4);

    #[quickcheck]
    fn dot_product(a: i32_4, b: i32_4, Small(alpha): Small<i32>) -> bool {
        (i32_4::dot(&a, &b) == i32_4::dot(&b, &a))
            && (i32_4::dot(&a, &(b.clone() * alpha)) == alpha * i32_4::dot(&a, &b))
            && (i32_4::dot(&a, &i32_4::zero()) == 0)
    }
}
