use crate::linear::*;
use crate::numeric::*;
use crate::tracing::*;

pub struct SimplePbMaterial<N: Num> {
    pub wavelength_peak: N,
    pub wavelength_width: N,
    pub roughness: N,
    pub shininess: N,
}

impl<N: Num, R: Random<N>> Material<R> for SimplePbMaterial<N> {
    type Num = N;
    type Meta = ();

    fn trace_reflection(
        &self,
        mut fwd_uv_ray: Ray<Self::Num>,
        mut light: Light<Self::Num>,
        rng: &mut R,
    ) -> Option<SecondaryRay<Self::Num, Self::Meta>> {
        if (light.wavelength - self.wavelength_peak).abs() > self.wavelength_width {
            return None;
        }

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

        Some(SecondaryRay {
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
        if (light.wavelength - self.wavelength_peak).abs() > self.wavelength_width {
            return None;
        }

        let h0 = self.roughness * rng.random();
        let h1 = self.roughness * rng.random();
        let h2 = self.roughness * rng.random();

        let n = Vector3(h0 - h1, h0 - h2, N::one());

        let dotn = Vector3::dot(&bwd_uv_ray.direction, &n);
        light.intensity *= dotn.abs();

        Some(light)
    }
}
