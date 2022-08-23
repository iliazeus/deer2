use crate::math::*;

use std::intrinsics::likely;

use super::*;

pub struct TriangleList<N: Num> {
    pub triangles: Vec<Triangle<N>>,
}

impl<N: Num> TriangleList<N> {
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
        }
    }
}

impl<N: Num> From<Vec<Triangle<N>>> for TriangleList<N> {
    fn from(triangles: Vec<Triangle<N>>) -> Self {
        Self { triangles }
    }
}

impl<'a, N: Num> Castable<'a, N> for TriangleList<N> {
    fn cast_ray(&'a self, ray: Ray<N>, max_d: N) -> Option<RayIntersection<'a, N>> {
        let mut cur_d = max_d;
        let mut cur_tri: Option<&Triangle<N>> = None;
        let mut cur_p_abc = Vector3::ZERO;

        for tri in self.triangles.iter() {
            let src_abc = tri.m_abc * (ray.src - tri.a);
            let dir_abc = tri.m_abc * ray.dir1;

            if src_abc.z() < N::EPS || dir_abc.z() > N::EPS {
                continue;
            }

            let d = -src_abc.z() / dir_abc.z();

            if d >= cur_d {
                continue;
            }

            let p_abc = src_abc + dir_abc * d;

            if likely(
                p_abc.x() < -N::EPS
                    || p_abc.y() < -N::EPS
                    || p_abc.x() + p_abc.y() > N::ONE + N::EPS,
            ) {
                continue;
            }

            cur_d = d;
            cur_tri = Some(tri);
            cur_p_abc = p_abc;
        }

        if let Some(cur_tri) = cur_tri {
            Some(RayIntersection {
                tri: cur_tri,
                d: cur_d,
                p_abc: cur_p_abc,
            })
        } else {
            None
        }
    }
}
