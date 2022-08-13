use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub trait Num:
    Copy
    + Display
    + PartialEq<Self>
    + PartialOrd<Self>
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
}

pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

macro_rules! impl_zero {
    ($T:ident) => {
        impl Zero for $T {
            #[inline(always)]
            fn zero() -> Self {
                0 as $T
            }
        }
    };
}

macro_rules! impl_one {
    ($T:ident) => {
        impl One for $T {
            #[inline(always)]
            fn one() -> Self {
                1 as $T
            }
        }
    };
}

impl_zero!(u8);
impl_zero!(u16);
impl_zero!(u32);
impl_zero!(u64);

impl_zero!(i8);
impl_zero!(i16);
impl_zero!(i32);
impl_zero!(i64);

impl_zero!(usize);
impl_zero!(isize);

impl_zero!(f32);
impl_zero!(f64);

impl_one!(u8);
impl_one!(u16);
impl_one!(u32);
impl_one!(u64);

impl_one!(i8);
impl_one!(i16);
impl_one!(i32);
impl_one!(i64);

impl_one!(usize);
impl_one!(isize);

impl_one!(f32);
impl_one!(f64);

impl Num for u8 {}
impl Num for u16 {}
impl Num for u32 {}
impl Num for u64 {}

impl Num for i8 {}
impl Num for i16 {}
impl Num for i32 {}
impl Num for i64 {}

impl Num for usize {}
impl Num for isize {}

impl Num for f32 {}
impl Num for f64 {}
