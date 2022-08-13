#![allow(non_camel_case_types)]

use crate::numeric::*;

use super::*;

use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Row-based.
/// Multiplying a vector and a matrix consumes the vector.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Matrix4<T: Num>(
    pub Vector4<T>,
    pub Vector4<T>,
    pub Vector4<T>,
    pub Vector4<T>,
);

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

impl<T: Num> Mul<&Matrix4<T>> for &Matrix4<T> {
    type Output = Matrix4<T>;
    fn mul(self, rhs: &Matrix4<T>) -> Matrix4<T> {
        macro_rules! mul {
            ($i:tt, $j:tt) => {
                (self.$i.0 * rhs.0.$j
                    + self.$i.1 * rhs.1.$j
                    + self.$i.2 * rhs.2.$j
                    + self.$i.3 * rhs.3.$j)
            };
        }

        Matrix4(
            Vector4(mul!(0, 0), mul!(0, 1), mul!(0, 2), mul!(0, 3)),
            Vector4(mul!(1, 0), mul!(1, 1), mul!(1, 2), mul!(1, 3)),
            Vector4(mul!(2, 0), mul!(2, 1), mul!(2, 2), mul!(2, 3)),
            Vector4(mul!(3, 0), mul!(3, 1), mul!(3, 2), mul!(3, 3)),
        )
    }
}

impl<T: Num> MulAssign<&Self> for Matrix4<T> {
    fn mul_assign(&mut self, rhs: &Self) {
        *self = &*self * rhs;
    }
}

impl<T: Num> Mul<Vector4<T>> for &Matrix4<T> {
    type Output = Vector4<T>;
    #[inline(always)]
    fn mul(self, rhs: Vector4<T>) -> Vector4<T> {
        Vector4(
            Vector4::dot(&self.0, &rhs),
            Vector4::dot(&self.1, &rhs),
            Vector4::dot(&self.2, &rhs),
            Vector4::dot(&self.3, &rhs),
        )
    }
}

impl<T: Num> Matrix4<T> {
    pub fn transpose(&self) -> Self {
        Self(
            Vector4(self.0 .0, self.1 .0, self.2 .1, self.3 .0),
            Vector4(self.0 .1, self.1 .1, self.2 .1, self.3 .1),
            Vector4(self.0 .2, self.1 .2, self.2 .2, self.3 .2),
            Vector4(self.0 .3, self.1 .3, self.2 .3, self.3 .3),
        )
    }

    pub fn from_rows(v0: Vector4<T>, v1: Vector4<T>, v2: Vector4<T>, v3: Vector4<T>) -> Self {
        Self(v0, v1, v2, v3)
    }

    pub fn from_cols(v0: Vector4<T>, v1: Vector4<T>, v2: Vector4<T>, v3: Vector4<T>) -> Self {
        Self(v0, v1, v2, v3).transpose()
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

    use quickcheck_macros::quickcheck;

    LinearSpace_tests!(i32_4x4);

    #[quickcheck]
    fn matrix_multiplication(a: i32_4x4, b: i32_4x4, c: i32_4x4, Small(alpha): Small<i32>) -> bool {
        (&a * &i32_4x4::one() == a)
            && (&i32_4x4::one() * &a == a)
            && (&a * &i32_4x4::zero() == i32_4x4::zero())
            && (&i32_4x4::zero() * &a == i32_4x4::zero())
            && (&(a.clone() * alpha) * &b == (&a * &b) * alpha)
            && (&a * &(b.clone() * alpha) == (&a * &b) * alpha)
            && (&(&a * &b) * &c == &a * &(&b * &c))
    }

    #[quickcheck]
    fn vector_multiplication(m: i32_4x4, u: i32_4) -> bool {
        (&i32_4x4::one() * u.clone() == u)
            && (&i32_4x4::zero() * u.clone() == i32_4::zero())
            && (&m * i32_4::zero() == i32_4::zero())
    }
}
