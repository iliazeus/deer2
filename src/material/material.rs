use crate::light::*;
use crate::numeric::*;

pub trait Material<R: Random<Self::Num>> {
    type Num: Num;
    type Meta;

    /// If an incoming ray reflected as `fwd_uv_ray` carrying `light`,
    /// what was the original ray, and what light did it carry?
    fn trace_reflection(
        &self,
        fwd_uv_ray: Ray<Self::Num>,
        light: Light<Self::Num>,
        rng: &mut R,
    ) -> Option<TracedRay<Self::Num, Self::Meta>>;

    /// If an incoming ray that was an opposite of `bwd_uv_ray` reflected as `fwd_uv_ray`
    /// carrying `light`, what light did it carry before the reflection?
    /// Used for direct light sampling.
    fn query_reflection(
        &self,
        bwd_uv_ray: Ray<Self::Num>,
        fwd_uv_ray: Ray<Self::Num>,
        light: Light<Self::Num>,
        rng: &mut R,
    ) -> Option<Light<Self::Num>>;
}

#[derive(Debug, Clone)]
pub struct TracedRay<N: Num, M> {
    pub ray: Ray<N>,
    pub light: Light<N>,
    pub meta: M,
}
