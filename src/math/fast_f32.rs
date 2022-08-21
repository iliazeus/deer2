#![allow(non_camel_case_types)]

use std::fmt::*;
use std::intrinsics::*;
use std::ops::*;

use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
#[repr(transparent)]
pub struct ff32(pub f32);

impl Display for ff32 {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(&self.0, f)
    }
}

impl Num for ff32 {
    #[inline(always)]
    fn from_usize(x: usize) -> Self {
        ff32(x as f32)
    }

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
    #[inline(always)]
    fn zero() -> ff32 {
        ff32(0.0)
    }
}

impl One for ff32 {
    #[inline(always)]
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

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;
    use test::Bencher;

    fn generic_bench<N: Num, const SIZE: usize>() -> N {
        let mut accum = N::zero();

        for i in 0..SIZE {
            let v1 = Vector3(N::from_usize(i + 0), N::from_usize(i + 1), N::from_usize(i + 2));
            let v2 = Vector3(N::from_usize(i + 3), N::from_usize(i + 4), N::from_usize(i + 5));
            let v3 = Vector3(N::from_usize(i + 6), N::from_usize(i + 7), N::from_usize(i + 8));

            let m = Matrix3(v1, v2, v3);
            accum += (m * m).det();
        }

        accum
    }

    #[bench]
    fn bench_ff32(b: &mut Bencher) {
        b.iter(|| generic_bench::<ff32, 1_000_000>())
    }

    #[bench]
    fn bench_f32(b: &mut Bencher) {
        b.iter(|| generic_bench::<f32, 1_000_000>())
    }

    #[bench]
    fn bench_f64(b: &mut Bencher) {
        b.iter(|| generic_bench::<f64, 1_000_000>())
    }
}
