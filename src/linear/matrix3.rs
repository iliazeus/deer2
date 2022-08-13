#![allow(non_camel_case_types)]

use crate::numeric::*;

use super::*;

use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Row-based.
/// Multiplying a vector and a matrix consumes the vector.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Matrix3<T: Num>(pub Vector3<T>, pub Vector3<T>, pub Vector3<T>);

pub type u8_3x3 = Matrix3<u8>;
pub type u16_3x3 = Matrix3<u16>;
pub type u32_3x3 = Matrix3<u32>;
pub type u64_3x3 = Matrix3<u64>;

pub type i8_3x3 = Matrix3<i8>;
pub type i16_3x3 = Matrix3<i16>;
pub type i32_3x3 = Matrix3<i32>;
pub type i64_3x3 = Matrix3<i64>;

pub type usize_3x3 = Matrix3<usize>;
pub type isize_3x3 = Matrix3<isize>;

pub type f32_3x3 = Matrix3<f32>;
pub type f64_3x3 = Matrix3<f64>;

impl<T: Num> LinearSpace for Matrix3<T> {
    type Scalar = T;
}

macro_rules! do_3 {
    ($lhs:ident.i $op:tt $rhs:ident) => {
        $lhs.0 $op $rhs;
        $lhs.1 $op $rhs;
        $lhs.2 $op $rhs;
    };

    ($lhs:ident.i $op:tt &$rhs:ident.i) => {
        $lhs.0 $op &$rhs.0;
        $lhs.1 $op &$rhs.1;
        $lhs.2 $op &$rhs.2;
    };
}

macro_rules! self_from_3 {
    ($arg:expr) => {
        Self($arg, $arg, $arg)
    };
}

impl<T: Num> Display for Matrix3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !f.alternate() {
            write!(f, "[{}, {}, {}]", self.0, self.1, self.2)
        } else {
            write!(f, "[{},\n {},\n {}]", self.0, self.1, self.2)
        }
    }
}

impl<T: Num> Add<&Self> for Matrix3<T> {
    type Output = Self;
    #[inline(always)]
    fn add(mut self, rhs: &Self) -> Self {
        do_3!(self.i += &rhs.i);
        self
    }
}

impl<T: Num> Sub<&Self> for Matrix3<T> {
    type Output = Self;
    #[inline(always)]
    fn sub(mut self, rhs: &Self) -> Self {
        do_3!(self.i -= &rhs.i);
        self
    }
}

impl<T: Num> AddAssign<&Self> for Matrix3<T> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: &Self) {
        do_3!(self.i += &rhs.i);
    }
}

impl<T: Num> SubAssign<&Self> for Matrix3<T> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: &Self) {
        do_3!(self.i -= &rhs.i);
    }
}

impl<T: Num> Mul<T> for Matrix3<T> {
    type Output = Self;
    #[inline(always)]
    fn mul(mut self, rhs: T) -> Self {
        do_3!(self.i *= rhs);
        self
    }
}

impl<T: Num> Div<T> for Matrix3<T> {
    type Output = Self;
    #[inline(always)]
    fn div(mut self, rhs: T) -> Self {
        do_3!(self.i /= rhs);
        self
    }
}

impl<T: Num> MulAssign<T> for Matrix3<T> {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: T) {
        do_3!(self.i *= rhs);
    }
}

impl<T: Num> DivAssign<T> for Matrix3<T> {
    #[inline(always)]
    fn div_assign(&mut self, rhs: T) {
        do_3!(self.i /= rhs);
    }
}

impl<T: Num> Zero for Matrix3<T> {
    #[inline(always)]
    fn zero() -> Self {
        self_from_3!(Vector3::<T>::zero())
    }
}

impl<T: Num> One for Matrix3<T> {
    #[inline(always)]
    fn one() -> Self {
        Self(Vector3::e0(), Vector3::e1(), Vector3::e2())
    }
}

impl<T: Num> Mul<&Matrix3<T>> for &Matrix3<T> {
    type Output = Matrix3<T>;
    fn mul(self, rhs: &Matrix3<T>) -> Matrix3<T> {
        macro_rules! mul {
            ($i:tt, $j:tt) => {
                (self.$i.0 * rhs.0.$j + self.$i.1 * rhs.1.$j + self.$i.2 * rhs.2.$j)
            };
        }

        Matrix3(
            Vector3(mul!(0, 0), mul!(0, 1), mul!(0, 2)),
            Vector3(mul!(1, 0), mul!(1, 1), mul!(1, 2)),
            Vector3(mul!(2, 0), mul!(2, 1), mul!(2, 2)),
        )
    }
}

impl<T: Num> MulAssign<&Self> for Matrix3<T> {
    fn mul_assign(&mut self, rhs: &Self) {
        *self = &*self * rhs;
    }
}

impl<T: Num> Mul<Vector3<T>> for &Matrix3<T> {
    type Output = Vector3<T>;
    #[inline(always)]
    fn mul(self, rhs: Vector3<T>) -> Vector3<T> {
        Vector3(
            Vector3::dot(&self.0, &rhs),
            Vector3::dot(&self.1, &rhs),
            Vector3::dot(&self.2, &rhs),
        )
    }
}

impl<T: Num> Matrix3<T> {
    pub fn transpose(&self) -> Self {
        Self(
            Vector3(self.0 .0, self.1 .0, self.2 .1),
            Vector3(self.0 .1, self.1 .1, self.2 .1),
            Vector3(self.0 .2, self.1 .2, self.2 .2),
        )
    }

    pub fn from_rows(v0: Vector3<T>, v1: Vector3<T>, v2: Vector3<T>) -> Self {
        Self(v0, v1, v2)
    }

    pub fn from_cols(v0: Vector3<T>, v1: Vector3<T>, v2: Vector3<T>) -> Self {
        Self(v0, v1, v2).transpose()
    }
}

#[cfg(test)]
use quickcheck::Arbitrary;

#[cfg(test)]
impl Arbitrary for Matrix3<i32> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        self_from_3!(Vector3::<i32>::arbitrary(g))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use quickcheck_macros::quickcheck;

    LinearSpace_tests!(i32_3x3);

    #[quickcheck]
    fn matrix_multiplication(a: i32_3x3, b: i32_3x3, c: i32_3x3, Small(alpha): Small<i32>) -> bool {
        (&a * &i32_3x3::one() == a)
            && (&i32_3x3::one() * &a == a)
            && (&a * &i32_3x3::zero() == i32_3x3::zero())
            && (&i32_3x3::zero() * &a == i32_3x3::zero())
            && (&(a.clone() * alpha) * &b == (&a * &b) * alpha)
            && (&a * &(b.clone() * alpha) == (&a * &b) * alpha)
            && (&(&a * &b) * &c == &a * &(&b * &c))
    }

    #[quickcheck]
    fn vector_multiplication(m: i32_3x3, u: i32_3) -> bool {
        (&i32_3x3::one() * u.clone() == u)
            && (&i32_3x3::zero() * u.clone() == i32_3::zero())
            && (&m * i32_3::zero() == i32_3::zero())
    }
}
