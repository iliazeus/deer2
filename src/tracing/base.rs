use crate::affine::*;
use crate::geometry::*;
use crate::linear::*;
use crate::numeric::*;

#[derive(Debug, Clone)]
pub struct Ray<N: Num> {
    pub origin: Vector3<N>,
    pub direction: Vector3<N>,
}

impl<N: Num> Geometry for Ray<N> {
    type Num = N;

    fn apply(mut self, xform: &Transform3<N>) -> Self {
        self.origin = xform.map_point(self.origin);
        self.direction = xform.map_point(self.direction);
        self
    }
}

pub trait CastRay {
    type Num: Num;
    type Meta;

    fn cast_ray(&self, bwd_ray: Ray<Self::Num>) -> Option<SurfacePoint<Self::Num, Self::Meta>>;
}

#[derive(Debug, Clone)]
pub struct SurfacePoint<N: Num, M> {
    pub fwd_uv_ray: Ray<N>,
    pub inv_uv_xform: Transform3<N>,
    pub meta: M,
}

#[derive(Debug, Clone)]
pub struct Light<N: Num> {
    pub wavelength: N,
    pub intensity: N,
}

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
    ) -> Option<SecondaryRay<Self::Num, Self::Meta>>;

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
pub struct SecondaryRay<N: Num, M> {
    pub ray: Ray<N>,
    pub light: Light<N>,
    pub meta: M,
}

pub trait LightSource<R: Random<Self::Num>> {
    type Num: Num;

    fn cast_ray_from(&self, origin: Vector3<Self::Num>, rng: &mut R) -> Option<Ray<Self::Num>>;
    fn get_exposure(&self, fwd_ray: Ray<Self::Num>, rng: &mut R) -> Option<Self::Num>;
}
