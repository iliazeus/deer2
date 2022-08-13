use crate::numeric::{Num, One, Zero};

use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// All binary ops consume the left-hand side.
/// This, as well as not requiring `Copy`, are deliberate decisions.
pub trait LinearSpace:
    Display
    + Clone
    + PartialEq<Self>
    + PartialOrd<Self>
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> AddAssign<&'a Self>
    + for<'a> SubAssign<&'a Self>
    + Mul<Self::Scalar, Output = Self>
    + Div<Self::Scalar, Output = Self>
    + MulAssign<Self::Scalar>
    + DivAssign<Self::Scalar>
    + Zero
    + One
{
    type Scalar: Num;
}

#[cfg(test)]
#[macro_export]
macro_rules! LinearSpace_tests {
    ($T:ident) => {
        mod linear_space {
            use super::*;

            use quickcheck_macros::quickcheck;

            type S = <$T as LinearSpace>::Scalar;

            #[quickcheck]
            fn addition(a: $T, b: $T, c: $T) -> bool {
                (a.clone() + &$T::zero() == a.clone())
                    && (a.clone() + &b == b.clone() + &a)
                    && ((a.clone() + &b) + &c == a.clone() + &(b.clone() + &c))
            }

            #[quickcheck]
            fn subtraction(a: $T, b: $T) -> bool {
                (a.clone() - &a + &a == a.clone())
                    && (a.clone() - &b == $T::zero() - &(b.clone() - &a))
                    && (a.clone() + &$T::zero() == a.clone())
            }

            #[quickcheck]
            fn scalar_multiplication(a: $T, Small(alpha): Small<S>, Small(beta): Small<S>) -> bool {
                (a.clone() * S::zero() == $T::zero())
                    && (a.clone() * S::one() == a.clone())
                    && ((a.clone() * alpha) * beta == a.clone() * (alpha * beta))
            }

            #[quickcheck]
            fn scalar_division(a: $T, Small(alpha): Small<S>) -> bool {
                (a.clone() * 1 == a.clone())
                    && (a.clone() * 0 == $T::zero())
                    && (a.clone() * alpha / alpha == a.clone())
            }
        }
    };
}
