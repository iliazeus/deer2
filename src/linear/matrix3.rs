#![allow(non_camel_case_types)]

use crate::numeric::*;

use super::*;

use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Row-based.
/// Multiplying a vector and a matrix consumes the vector.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Matrix3<T: Num>(pub Vector3<T>, pub Vector3<T>, pub Vector3<T>);

pub type i8_3x3 = Matrix3<i8>;
pub type i16_3x3 = Matrix3<i16>;
pub type i32_3x3 = Matrix3<i32>;
pub type i64_3x3 = Matrix3<i64>;

pub type isize_3x3 = Matrix3<isize>;

pub type f32_3x3 = Matrix3<f32>;
pub type f64_3x3 = Matrix3<f64>;

pub type r64_3x3 = Matrix3<r64>;

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

impl<T: Num> Neg for Matrix3<T> {
    type Output = Self;
    #[inline(always)]
    fn neg(mut self) -> Self::Output {
        self.0 = -self.0;
        self.1 = -self.1;
        self.2 = -self.2;
        self
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
    pub fn tr(&self) -> Self {
        Self(
            Vector3(self.0 .0, self.1 .0, self.2 .1),
            Vector3(self.0 .1, self.1 .1, self.2 .1),
            Vector3(self.0 .2, self.1 .2, self.2 .2),
        )
    }

    pub fn from_rows(v0: Vector3<T>, v1: Vector3<T>, v2: Vector3<T>) -> Self {
        Self(v0, v1, v2)
    }

    pub fn from_cols(e0: Vector3<T>, e1: Vector3<T>, e2: Vector3<T>) -> Self {
        Self(e0, e1, e2).tr()
    }
}

impl<T: Num> Matrix3<T> {
    pub fn det(&self) -> T {
        let d00 = T::zero() + (self.1 .1 * self.2 .2 - self.1 .2 * self.2 .1);
        let d01 = T::zero() - (self.1 .0 * self.2 .2 - self.1 .2 * self.2 .0);
        let d02 = T::zero() + (self.0 .0 * self.1 .1 - self.0 .1 * self.1 .0);

        let det = self.0 .0 * d00 + self.0 .1 * d01 + self.0 .2 * d02;

        det
    }

    pub fn inv(&self) -> Option<Self> {
        let d00 = T::zero() + (self.1 .1 * self.2 .2 - self.1 .2 * self.2 .1);
        let d01 = T::zero() - (self.1 .0 * self.2 .2 - self.1 .2 * self.2 .0);
        let d02 = T::zero() + (self.1 .0 * self.2 .1 - self.1 .1 * self.2 .0);

        let d10 = T::zero() - (self.0 .1 * self.2 .2 - self.0 .2 * self.2 .1);
        let d11 = T::zero() + (self.0 .0 * self.2 .2 - self.0 .2 * self.2 .0);
        let d12 = T::zero() - (self.0 .0 * self.2 .1 - self.0 .1 * self.2 .0);

        let d20 = T::zero() + (self.0 .1 * self.1 .2 - self.0 .2 * self.1 .1);
        let d21 = T::zero() - (self.0 .0 * self.1 .2 - self.0 .2 * self.1 .0);
        let d22 = T::zero() + (self.0 .0 * self.1 .1 - self.0 .1 * self.1 .0);

        let det = self.0 .0 * d00 + self.0 .1 * d01 + self.0 .2 * d02;

        if det == T::zero() {
            return None;
        }

        let result = Self(
            Vector3(d00 / det, d10 / det, d20 / det),
            Vector3(d01 / det, d11 / det, d21 / det),
            Vector3(d02 / det, d12 / det, d22 / det),
        );

        Some(result)
    }
}

#[cfg(test)]
use quickcheck::Arbitrary;

#[cfg(test)]
impl Arbitrary for Matrix3<r64> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        self_from_3!(Vector3::<r64>::arbitrary(g))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use quickcheck_macros::quickcheck;

    LinearSpace_tests!(Matrix3);

    #[quickcheck]
    fn matrix_multiplication(a: r64_3x3, b: r64_3x3, c: r64_3x3, alpha: r64) -> bool {
        (&a * &r64_3x3::one() == a)
            && (&r64_3x3::one() * &a == a)
            && (&a * &r64_3x3::zero() == r64_3x3::zero())
            && (&r64_3x3::zero() * &a == r64_3x3::zero())
            && (&(a.clone() * alpha) * &b == (&a * &b) * alpha)
            && (&a * &(b.clone() * alpha) == (&a * &b) * alpha)
            && (&(&a * &b) * &c == &a * &(&b * &c))
    }

    #[quickcheck]
    fn vector_multiplication(m: r64_3x3, u: r64_3) -> bool {
        (&r64_3x3::one() * u.clone() == u)
            && (&r64_3x3::zero() * u.clone() == r64_3::zero())
            && (&m * r64_3::zero() == r64_3::zero())
    }

    #[quickcheck]
    fn inversion(m: r64_3x3) -> bool {
        (r64_3x3::one().inv() == Some(r64_3x3::one()))
            && (r64_3x3::zero().inv() == None)
            && (m.inv().is_none() || m.inv().unwrap().inv().unwrap() == m)
    }
}
