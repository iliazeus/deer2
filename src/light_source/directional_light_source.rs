use crate::light::*;
use crate::linear::*;
use crate::numeric::*;

use super::*;

pub struct DirectionalLightSource<N: Num, S: Spectrum<N>> {
    pub spectrum: S,
    pub direction: Vector3<N>,
}

impl<N: Num, S: Spectrum<N>, R: Random<N>> LightSource<R> for DirectionalLightSource<N, S> {
    type Num = N;

    fn cast_ray_from(&self, origin: Vector3<N>, _rng: &mut R) -> Option<Ray<N>> {
        Some(Ray {
            origin,
            direction: Vector3::zero() - &self.direction,
        })
    }

    fn get_exposure(&self, _fwd_ray: Ray<N>, light: Light<N>, _rng: &mut R) -> Option<N> {
        self.spectrum.get_intensity(light.wavelength)
    }
}
