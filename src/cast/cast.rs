use crate::affine::*;
use crate::light::*;
use crate::numeric::*;

pub trait CastRay {
    type Num: Num;
    type MaterialMeta;

    fn cast_ray(&self, bwd_ray: Ray<Self::Num>) -> Option<SurfacePoint<Self::Num, Self::MaterialMeta>>;
}

#[derive(Debug, Clone)]
pub struct SurfacePoint<N: Num, M> {
    pub fwd_uv_ray: Ray<N>,
    pub uv_xform: Transform3<N>,
    pub inv_uv_xform: Transform3<N>,
    pub meta: M,
}
