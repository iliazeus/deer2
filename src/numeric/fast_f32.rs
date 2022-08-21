#![allow(non_camel_case_types)]

use std::fmt::*;
use std::intrinsics::*;
use std::ops::*;

use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct ff32(pub f32);

impl Display for ff32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(&self.0, f)
    }
}

impl Num for ff32 {
    #[inline(always)]
    fn abs(self) -> ff32 {
        ff32(self.0.abs())
    }

    #[inline(always)]
    fn sqrt(self) -> ff32 {
        ff32(self.0.sqrt())
    }

    #[inline(always)]
    fn sin(self) -> ff32 {
        ff32(self.0.sin())
    }

    #[inline(always)]
    fn cos(self) -> ff32 {
        ff32(self.0.cos())
    }
}

impl Zero for ff32 {
    fn zero() -> ff32 {
        ff32(0.0)
    }
}

impl One for ff32 {
    fn one() -> ff32 {
        ff32(1.0)
    }
}

impl Neg for ff32 {
    type Output = ff32;

    #[inline(always)]
    fn neg(self) -> ff32 {
        unsafe { ff32(fsub_fast(0.0, self.0)) }
    }
}

impl Add<ff32> for ff32 {
    type Output = ff32;

    #[inline(always)]
    fn add(self, rhs: ff32) -> ff32 {
        unsafe { ff32(fadd_fast(self.0, rhs.0)) }
    }
}

impl Sub<ff32> for ff32 {
    type Output = ff32;

    #[inline(always)]
    fn sub(self, rhs: ff32) -> ff32 {
        unsafe { ff32(fsub_fast(self.0, rhs.0)) }
    }
}

impl Mul<ff32> for ff32 {
    type Output = ff32;

    #[inline(always)]
    fn mul(self, rhs: ff32) -> ff32 {
        unsafe { ff32(fmul_fast(self.0, rhs.0)) }
    }
}

impl Div<ff32> for ff32 {
    type Output = ff32;

    #[inline(always)]
    fn div(self, rhs: ff32) -> ff32 {
        unsafe { ff32(fdiv_fast(self.0, rhs.0)) }
    }
}

impl AddAssign<ff32> for ff32 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: ff32) {
        *self = *self + rhs
    }
}

impl SubAssign<ff32> for ff32 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: ff32) {
        *self = *self - rhs
    }
}

impl MulAssign<ff32> for ff32 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: ff32) {
        *self = *self * rhs
    }
}

impl DivAssign<ff32> for ff32 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: ff32) {
        *self = *self / rhs
    }
}
