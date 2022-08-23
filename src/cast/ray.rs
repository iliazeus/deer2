use crate::math::*;

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray<N: Num> {
    /// ray source
    pub src: Vector3<N>,

    /// unit-length ray direction
    pub dir1: Vector3<N>,
}

#[derive(Debug)]
pub struct RayIntersection<'a, N: Num> {
    /// triangle
    pub tri: &'a Triangle<N>,

    /// distance from origin along the ray
    pub d: N,

    /// intersection point in (AB, AC, N1) space
    pub p_abc: Vector3<N>,
}

impl<'a, N: Num> RayIntersection<'a, N> {
    pub fn interpolate_meta(&self) -> InterpolatedMeta<N> {
        let w = Vector3::new(
            N::ONE - self.p_abc.x() - self.p_abc.y(),
            self.p_abc.x(),
            self.p_abc.y(),
        );

        InterpolatedMeta {
            w,
            n1_p: (self.tri.meta.abc_nc * w).norm(),
            p_uv: self.tri.meta.abc_uv * w,
        }
    }
}
