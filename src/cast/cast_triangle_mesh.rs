use crate::geometry::*;
use crate::numeric::*;

use super::*;

pub fn cast_triangle_mesh<'a, N, R, T, TM>(ray: R, mesh: &'a TM) -> Option<SurfacePoint<N>>
where
    N: Num,
    R: Ray<Num = N>,
    T: Triangle<Num = N> + 'a,
    TM: TriangleMesh<'a, Triangle = T>,
{
    let mut result: Option<SurfacePoint<N>> = None;
    let mut result_dist2: Option<N> = None;

    for triangle in mesh.triangles() {
        if let Some(candidate) = cast_triangle(ray.clone(), triangle) {
            let candidate_dist2 = (ray.source() - &candidate.point).abs2();

            if let Some(result_dist2) = result_dist2 && result_dist2 <= candidate_dist2 {
                continue;
            }

            result = Some(candidate);
            result_dist2 = Some(candidate_dist2);
        }
    }

    result
}
