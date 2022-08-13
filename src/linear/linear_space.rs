use crate::numeric::Num;

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
{
    type Scalar: Num;
}
