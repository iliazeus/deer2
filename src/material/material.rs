use crate::light::*;
use crate::numeric::*;

pub trait Material {
    type Num: Num;
    type Meta;

    /// If an incoming ray reflected as `fwd_uv_ray` carrying `light`,
    /// what was the original ray, and what light did it carry?
    fn trace_reflection<R: Random<Self::Num>>(
        &self,
        fwd_uv_ray: Ray<Self::Num>,
        light: Light<Self::Num>,
        rng: &mut R,
        meta: &Self::Meta,
    ) -> Option<(Ray<Self::Num>, Light<Self::Num>)>;

    /// If an incoming ray that was an opposite of `bwd_uv_ray` reflected as `fwd_uv_ray`
    /// carrying `light`, what light did it carry before the reflection?
    /// Used for direct light sampling.
    fn query_reflection<R: Random<Self::Num>>(
        &self,
        bwd_uv_ray: Ray<Self::Num>,
        fwd_uv_ray: Ray<Self::Num>,
        light: Light<Self::Num>,
        rng: &mut R,
        meta: &Self::Meta,
    ) -> Option<Light<Self::Num>>;
}
