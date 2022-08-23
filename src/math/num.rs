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
    const EPS: Self;
    const PI: Self;

    fn from_usize(x: usize) -> Self;

    fn abs(self) -> Self;
    fn sqrt(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
}

pub trait Zero {
    const ZERO: Self;
}

pub trait One {
    const ONE: Self;
}

macro_rules! impl_float {
    ($T:ident, eps = $eps:tt) => {
        impl Zero for $T {
            const ZERO: Self = 0 as $T;
        }

        impl One for $T {
            const ONE: Self = 1 as $T;
        }

        impl Num for $T {
            const EPS: Self = $eps;
            const PI: Self = std::$T::consts::PI;

            #[inline(always)]
            fn from_usize(x: usize) -> Self {
                x as Self
            }

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

impl_float!(f32, eps = 1e-4);
impl_float!(f64, eps = 1e-6);
