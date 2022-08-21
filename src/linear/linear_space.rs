use crate::numeric::{Num, One, Zero};

use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// We only really need to work with `ff32_3` and `ff32_3x3`,
/// both of which fit into a cache line. That is why we require `Copy`.
pub trait LinearSpace:
    Display
    + Clone
    + Copy
    + PartialEq<Self>
    + PartialOrd<Self>
    + Neg<Output = Self>
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
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
macro_rules! LinearSpace_tests {
    ($T:ident) => {
        mod linear_space {
            use super::*;

            use crate::numeric::r64;

            use quickcheck_macros::quickcheck;

            #[quickcheck]
            fn addition(a: $T<r64>, b: $T<r64>, c: $T<r64>) -> bool {
                (a + $T::zero() == a) && (a + b == b + a) && ((a + b) + c == a + (b + c))
            }

            #[quickcheck]
            fn subtraction(a: $T<r64>, b: $T<r64>) -> bool {
                (a - a + a == a) && (a - b == $T::zero() - (b - a)) && (a + $T::zero() == a)
            }

            #[quickcheck]
            fn scalar_multiplication(a: $T<r64>, alpha: r64, beta: r64) -> bool {
                (a * r64::zero() == $T::zero())
                    && (a * r64::one() == a)
                    && ((a * alpha) * beta == a * (alpha * beta))
            }

            #[quickcheck]
            fn scalar_division(a: $T<r64>, alpha: r64) -> bool {
                (a * r64::one() == a)
                    && (a * r64::zero() == $T::zero())
                    && ((a * alpha) / alpha == a)
            }
        }
    };
}
