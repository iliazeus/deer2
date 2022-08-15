use crate::affine::*;
use crate::geometry::*;
use crate::linear::*;
use crate::numeric::*;

use super::*;

pub fn cast_triangle<N, R, T>(ray: R, triangle: &T) -> Option<SurfacePoint<N>>
where
    N: Num,
    R: Ray<Num = N>,
    T: Triangle<Num = N>,
{
    let e0 = triangle.vertex_b() - &triangle.vertex_a();
    let e1 = triangle.vertex_c() - &triangle.vertex_a();
    let e2 = triangle.normal();

    let origin = triangle.vertex_a();

    let xform = Transform3::new(Matrix3::from_cols(e0, e1, e2), origin)?;

    let mut xray = ray.clone();
    xray.apply(&xform);

    let xsrc = ray.source();
    let xdir = ray.direction();

    if xsrc.z() == N::zero() {
        return Some(SurfacePoint {
            point: ray.source(),
            normal: triangle.normal(),
        });
    }

    if xdir.z() >= N::zero() {
        return None;
    }

    let xpoint_x = xsrc.x() + xdir.x() * xsrc.z() / xdir.z();
    let xpoint_y = xsrc.y() + xdir.y() * xsrc.z() / xdir.z();

    if xpoint_x < N::zero() || xpoint_y < N::zero() || xpoint_x + xpoint_y > N::one() {
        return None;
    }

    let inv_xform = xform.invert();

    let mut point = Vector3::new(xpoint_x, xpoint_y, N::zero());
    inv_xform.apply_to_point(&mut point);

    Some(SurfacePoint {
        point,
        normal: triangle.normal(),
    })
}
