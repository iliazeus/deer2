use crate::math::*;

use super::*;

pub trait Castable<'a, N: Num> {
    fn cast_ray(&'a self, ray: Ray<N>, max_d: N) -> Option<RayIntersection<'a, N>>;
}
