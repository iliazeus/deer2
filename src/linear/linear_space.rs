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

            use crate::numeric::r64;

            use quickcheck_macros::quickcheck;

            #[quickcheck]
            fn addition(a: $T<r64>, b: $T<r64>, c: $T<r64>) -> bool {
                (a.clone() + &$T::zero() == a.clone())
                    && (a.clone() + &b == b.clone() + &a)
                    && ((a.clone() + &b) + &c == a.clone() + &(b.clone() + &c))
            }

            #[quickcheck]
            fn subtraction(a: $T<r64>, b: $T<r64>) -> bool {
                (a.clone() - &a + &a == a.clone())
                    && (a.clone() - &b == $T::zero() - &(b.clone() - &a))
                    && (a.clone() + &$T::zero() == a.clone())
            }

            #[quickcheck]
            fn scalar_multiplication(a: $T<r64>, alpha: r64, beta: r64) -> bool {
                (a.clone() * r64::zero() == $T::zero())
                    && (a.clone() * r64::one() == a.clone())
                    && ((a.clone() * alpha) * beta == a.clone() * (alpha * beta))
            }

            #[quickcheck]
            fn scalar_division(a: $T<r64>, alpha: r64) -> bool {
                (a.clone() * r64::one() == a.clone())
                    && (a.clone() * r64::zero() == $T::zero())
                    && (a.clone() * alpha / alpha == a.clone())
            }
        }
    };
}
