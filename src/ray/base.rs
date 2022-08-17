use crate::affine::*;
use crate::geometry::*;
use crate::linear::*;
use crate::numeric::*;
use crate::random::*;

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

pub trait Material<R: Rng> {
    type Num: Num;
    type Meta;
    type Distrib: Distrib<SecondaryRay<Self::Num, Self::Meta>, R>;

    fn cast_ray(&self, fwd_uv_ray: Ray<Self::Num>, light: Light<Self::Num>) -> Self::Distrib;
}

#[derive(Debug, Clone)]
pub struct SecondaryRay<N: Num, M> {
    pub ray: Ray<N>,
    pub light: Light<N>,
    pub meta: M,
}
