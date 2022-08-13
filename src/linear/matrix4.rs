#![allow(non_camel_case_types)]

use crate::numeric::*;

use super::*;

use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Matrix4<T: Num>(Vector4<T>, Vector4<T>, Vector4<T>, Vector4<T>);

pub type u8_4x4 = Matrix4<u8>;
pub type u16_4x4 = Matrix4<u16>;
pub type u32_4x4 = Matrix4<u32>;
pub type u64_4x4 = Matrix4<u64>;

pub type i8_4x4 = Matrix4<i8>;
pub type i16_4x4 = Matrix4<i16>;
pub type i32_4x4 = Matrix4<i32>;
pub type i64_4x4 = Matrix4<i64>;

pub type usize_4x4 = Matrix4<usize>;
pub type isize_4x4 = Matrix4<isize>;

pub type f32_4x4 = Matrix4<f32>;
pub type f64_4x4 = Matrix4<f64>;

impl<T: Num> LinearSpace for Matrix4<T> {
    type Scalar = T;
}

macro_rules! do_4 {
    ($lhs:ident.i $op:tt $rhs:ident) => {
        $lhs.0 $op $rhs;
        $lhs.1 $op $rhs;
        $lhs.2 $op $rhs;
        $lhs.3 $op $rhs;
    };

    ($lhs:ident.i $op:tt &$rhs:ident.i) => {
        $lhs.0 $op &$rhs.0;
        $lhs.1 $op &$rhs.1;
        $lhs.2 $op &$rhs.2;
        $lhs.3 $op &$rhs.3;
    };
}

macro_rules! self_from_4 {
    ($arg:expr) => {
        Self($arg, $arg, $arg, $arg)
    };
}

impl<T: Num> Display for Matrix4<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !f.alternate() {
            write!(f, "[{}, {}, {}, {}]", self.0, self.1, self.2, self.3)
        } else {
            write!(f, "[{},\n {},\n {},\n {}]", self.0, self.1, self.2, self.3)
        }
    }
}

impl<T: Num> Add<&Self> for Matrix4<T> {
    type Output = Self;
    #[inline(always)]
    fn add(mut self, rhs: &Self) -> Self {
        do_4!(self.i += &rhs.i);
        self
    }
}

impl<T: Num> Sub<&Self> for Matrix4<T> {
    type Output = Self;
    #[inline(always)]
    fn sub(mut self, rhs: &Self) -> Self {
        do_4!(self.i -= &rhs.i);
        self
    }
}

impl<T: Num> AddAssign<&Self> for Matrix4<T> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: &Self) {
        do_4!(self.i += &rhs.i);
    }
}

impl<T: Num> SubAssign<&Self> for Matrix4<T> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: &Self) {
        do_4!(self.i -= &rhs.i);
    }
}

impl<T: Num> Mul<T> for Matrix4<T> {
    type Output = Self;
    #[inline(always)]
    fn mul(mut self, rhs: T) -> Self {
        do_4!(self.i *= rhs);
        self
    }
}

impl<T: Num> Div<T> for Matrix4<T> {
    type Output = Self;
    #[inline(always)]
    fn div(mut self, rhs: T) -> Self {
        do_4!(self.i /= rhs);
        self
    }
}

impl<T: Num> MulAssign<T> for Matrix4<T> {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: T) {
        do_4!(self.i *= rhs);
    }
}

impl<T: Num> DivAssign<T> for Matrix4<T> {
    #[inline(always)]
    fn div_assign(&mut self, rhs: T) {
        do_4!(self.i /= rhs);
    }
}

impl<T: Num> Zero for Matrix4<T> {
    #[inline(always)]
    fn zero() -> Self {
        self_from_4!(Vector4::<T>::zero())
    }
}

impl<T: Num> One for Matrix4<T> {
    #[inline(always)]
    fn one() -> Self {
        Self(Vector4::e0(), Vector4::e1(), Vector4::e2(), Vector4::e3())
    }
}

#[cfg(test)]
use quickcheck::Arbitrary;

#[cfg(test)]
impl Arbitrary for Matrix4<i32> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        self_from_4!(Vector4::<i32>::arbitrary(g))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    LinearSpace_tests!(i32_4x4);
}
