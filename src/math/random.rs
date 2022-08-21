use rand::prelude::*;

use super::*;

pub trait Random<N: Num> {
    /// Uniformly-distributed in the [0; 1) range
    fn random(&mut self) -> N;
}

impl<R: Rng> Random<f32> for R {
    #[inline(always)]
    fn random(&mut self) -> f32 {
        self.gen()
    }
}

impl<R: Rng> Random<f64> for R {
    #[inline(always)]
    fn random(&mut self) -> f64 {
        self.gen()
    }
}

impl<R: Rng> Random<ff32> for R {
    #[inline(always)]
    fn random(&mut self) -> ff32 {
        ff32(self.gen())
    }
}
