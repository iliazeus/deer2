use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Num:
    Copy
    + Debug
    + Display
    + PartialEq<Self>
    + PartialOrd<Self>
    + Neg<Output = Self>
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<Self>
    + DivAssign<Self>
    + Zero
    + One
{
    fn abs(self) -> Self;
    fn sqrt(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
}

pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

macro_rules! impl_float {
    ($T:ident) => {
        impl Zero for $T {
            #[inline(always)]
            fn zero() -> Self {
                0 as $T
            }
        }

        impl One for $T {
            #[inline(always)]
            fn one() -> Self {
                1 as $T
            }
        }

        impl Num for $T {
            #[inline(always)]
            fn abs(self) -> Self {
                $T::abs(self)
            }

            #[inline(always)]
            fn sqrt(self) -> Self {
                $T::sqrt(self)
            }

            #[inline(always)]
            fn sin(self) -> Self {
                $T::sin(self)
            }

            #[inline(always)]
            fn cos(self) -> Self {
                $T::cos(self)
            }
        }
    };
}

impl_float!(f32);
impl_float!(f64);
