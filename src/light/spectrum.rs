use crate::numeric::*;

pub trait Spectrum<N: Num> {
    fn get_intensity(&self, wavelength: N) -> Option<N>;
}

impl<N: Num, F: Fn(N) -> Option<N>> Spectrum<N> for F {
    #[inline(always)]
    fn get_intensity(&self, wavelength: N) -> Option<N> {
        self(wavelength)
    }
}
