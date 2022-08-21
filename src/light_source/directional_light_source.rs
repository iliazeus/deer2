use crate::light::*;
use crate::linear::*;
use crate::numeric::*;

use super::*;

pub struct DirectionalLightSource<N: Num, S: Spectrum<N>> {
    pub spectrum: S,
    pub direction: Vector3<N>,
}

impl<N: Num, S: Spectrum<N>> LightSource for DirectionalLightSource<N, S> {
    type Num = N;

    fn cast_ray_from<R: Random<N>>(&self, origin: Vector3<N>, _rng: &mut R) -> Ray<N> {
        Ray {
            origin,
            direction: Vector3::zero() - &self.direction,
        }
    }

    fn get_exposure<R: Random<N>>(&self, _fwd_ray: Ray<N>, light: Light<N>, _rng: &mut R) -> N {
        light.intensity / self.spectrum.get_intensity(light.wavelength)
    }
}
