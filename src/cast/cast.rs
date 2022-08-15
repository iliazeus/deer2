use crate::geometry::*;
use crate::numeric::*;

use super::*;

pub trait Cast<N, R, G>
where
    N: Num,
    R: Ray<Num = N>,
    G: Geometry<Num = N>,
{
    fn cast(&self, ray: R, geometry: &G) -> Option<SurfacePoint<N>>;
}

impl<F, N, R, G> Cast<N, R, G> for F
where
    F: Fn(R, &G) -> Option<SurfacePoint<N>>,
    N: Num,
    R: Ray<Num = N>,
    G: Geometry<Num = N>,
{
    fn cast(&self, ray: R, geometry: &G) -> Option<SurfacePoint<N>> {
        self(ray, geometry)
    }
}
