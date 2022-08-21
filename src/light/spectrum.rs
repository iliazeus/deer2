use crate::numeric::*;

pub trait Spectrum<N: Num> {
    fn get_intensity(&self, wavelength: N) -> N;
}

impl<N: Num, F: Fn(N) -> N> Spectrum<N> for F {
    #[inline(always)]
    fn get_intensity(&self, wavelength: N) -> N {
        self(wavelength)
    }
}
