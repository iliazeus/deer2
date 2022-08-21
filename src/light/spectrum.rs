use crate::numeric::*;

use super::*;

pub trait Spectrum<N: Num> {
    fn get_intensity(&self, wavelength: N) -> Option<N>;

    #[inline(always)]
    fn map_light(&self, light: Light<N>) -> Option<Light<N>> {
        self.get_intensity(light.wavelength).map(|intensity| Light {
            wavelength: light.wavelength,
            intensity: light.intensity * intensity,
        })
    }
}

impl<N: Num, F: Fn(N) -> Option<N>> Spectrum<N> for F {
    #[inline(always)]
    fn get_intensity(&self, wavelength: N) -> Option<N> {
        self(wavelength)
    }
}
