use crate::light::*;
use crate::linear::*;
use crate::numeric::*;

use super::*;

pub struct SimplePbMaterial<N: Num, S: Spectrum<N>> {
    pub spectrum: S,
    pub roughness: N,
    pub shininess: N,
}

impl<N: Num, S: Spectrum<N>, R: Random<N>> Material<R> for SimplePbMaterial<N, S> {
    type Num = N;
    type Meta = ();

    fn trace_reflection(
        &self,
        mut fwd_uv_ray: Ray<Self::Num>,
        mut light: Light<Self::Num>,
        rng: &mut R,
    ) -> Option<TracedRay<Self::Num, Self::Meta>> {
        light.intensity /= self.spectrum.get_intensity(light.wavelength)?;

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

        Some(TracedRay {
            ray: bwd_uv_ray,
            light,
            meta: (),
        })
    }

    fn query_reflection(
        &self,
        bwd_uv_ray: Ray<Self::Num>,
        _fwd_uv_ray: Ray<Self::Num>,
        mut light: Light<Self::Num>,
        rng: &mut R,
    ) -> Option<Light<Self::Num>> {
        light.intensity /= self.spectrum.get_intensity(light.wavelength)?;

        let h0 = self.roughness * rng.random();
        let h1 = self.roughness * rng.random();
        let h2 = self.roughness * rng.random();

        let n = Vector3(h0 - h1, h0 - h2, N::one());

        let dotn = Vector3::dot(&bwd_uv_ray.direction, &n);
        light.intensity *= dotn.abs();

        Some(light)
    }
}
