use crate::light::*;
use crate::linear::*;
use crate::numeric::*;

use super::*;

pub struct SimplePbMaterial<N: Num, S: Spectrum<N>> {
    pub spectrum: S,
    pub roughness: N,
    pub shininess: N,
}

impl<N: Num, S: Spectrum<N>> Material for SimplePbMaterial<N, S> {
    type Num = N;
    type Meta = ();

    fn trace_reflection<R: Random<N>>(
        &self,
        mut fwd_uv_ray: Ray<N>,
        mut light: Light<N>,
        rng: &mut R,
        _meta: &(),
    ) -> Option<(Ray<N>, Light<N>)> {
        let reflectiveness = self.spectrum.get_intensity(light.wavelength);
        if reflectiveness == N::zero() {
            return None;
        }

        light.intensity /= reflectiveness;

        fwd_uv_ray.direction /= fwd_uv_ray.direction.abs2();

        let h0 = self.roughness * rng.random();
        let h1 = self.roughness * rng.random();
        let h2 = self.roughness * rng.random();

        let n = Vector3(h0 - h1, h0 - h2, N::one());

        if rng.random() > self.shininess {
            let dotn = Vector3::dot(&fwd_uv_ray.direction, &n);
            light.intensity *= dotn.abs();
        }

        let dist = fwd_uv_ray.direction.clone() - &n;

        let bwd_uv_ray = Ray {
            origin: fwd_uv_ray.origin,
            direction: fwd_uv_ray.direction - &dist - &dist,
        };

        Some((bwd_uv_ray, light))
    }

    fn query_reflection<R: Random<N>>(
        &self,
        bwd_uv_ray: Ray<N>,
        _fwd_uv_ray: Ray<N>,
        mut light: Light<N>,
        rng: &mut R,
        _meta: &(),
    ) -> Option<Light<N>> {
        let reflectiveness = self.spectrum.get_intensity(light.wavelength);
        if reflectiveness == N::zero() {
            return None;
        }

        light.intensity /= reflectiveness;

        let h0 = self.roughness * rng.random();
        let h1 = self.roughness * rng.random();
        let h2 = self.roughness * rng.random();

        let n = Vector3(h0 - h1, h0 - h2, N::one());

        let dotn = Vector3::dot(&bwd_uv_ray.direction, &n);
        light.intensity *= dotn.abs();

        Some(light)
    }
}
