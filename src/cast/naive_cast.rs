use crate::affine::*;
use crate::geometry::*;
use crate::light::*;
use crate::linear::*;
use crate::numeric::*;

use super::*;

pub struct NaiveCastTriangleMesh<'a, TM: TriangleMesh<'a>>(pub &'a TM);

impl<'a, N, T, TM> CastRay for NaiveCastTriangleMesh<'a, TM>
where
    N: Num,
    T: Triangle<Num = N>,
    TM: TriangleMesh<'a, Num = N, Triangle = T>,
{
    type Num = N;
    type Meta = ();

    fn cast_ray(&self, bwd_ray: Ray<N>) -> Option<SurfacePoint<N, ()>> {
        let triangle_mesh = self.0;

        let surface_points = triangle_mesh
            .triangles()
            .flat_map(|t| NaiveCastTriangle(t).cast_ray(bwd_ray.clone()).into_iter());

        let mut closest_surface_point: Option<SurfacePoint<N, ()>> = None;
        let mut closest_surface_point_dist: Option<N> = None;

        for surface_point in surface_points {
            let fwd_world_ray = surface_point
                .fwd_uv_ray
                .clone()
                .apply(&surface_point.inv_uv_xform);

            let surface_point_dist = (fwd_world_ray.origin - &bwd_ray.origin).abs2();

            if let Some(d) = closest_surface_point_dist && d <= surface_point_dist {
                continue;
            } else {
                closest_surface_point = Some(surface_point);
                closest_surface_point_dist = Some(surface_point_dist);
            }
        }

        closest_surface_point
    }
}

pub struct NaiveCastTriangle<'a, T: Triangle>(pub &'a T);

impl<'a, N, T> CastRay for NaiveCastTriangle<'a, T>
where
    N: Num,
    T: Triangle<Num = N>,
{
    type Num = N;
    type Meta = ();

    fn cast_ray(&self, bwd_ray: Ray<N>) -> Option<SurfacePoint<N, ()>> {
        let triangle = self.0;

        let uv_xform = Transform3::new(
            Matrix3::from_cols(
                triangle.vertex_b() - &triangle.vertex_a(),
                triangle.vertex_c() - &triangle.vertex_a(),
                triangle.normal(),
            ),
            triangle.vertex_a(),
        );

        let uv_ray = bwd_ray.apply(&uv_xform);

        let dz = uv_ray.direction.z();
        let h = uv_ray.origin.z();

        if dz == N::zero() {
            return None;
        }

        let alpha = h / dz;

        if alpha < N::zero() {
            return None;
        }

        let inv_uv_xform = uv_xform.invert()?;

        let fwd_uv_ray = Ray {
            origin: uv_ray.origin + &(uv_ray.direction.clone() * alpha),
            direction: -uv_ray.direction,
        };

        Some(SurfacePoint {
            fwd_uv_ray,
            inv_uv_xform,
            meta: (),
        })
    }
}
