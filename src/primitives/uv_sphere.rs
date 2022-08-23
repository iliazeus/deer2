use crate::cast::*;
use crate::math::*;

struct Vertex<N: Num> {
    /// latitude
    lat: N,

    /// longitude
    long: N,

    /// vertex coords
    p: Vector3<N>,

    /// vertex normal
    nc: Vector3<N>,
}

impl<N: Num> Vertex<N> {
    fn new(center: Vector3<N>, radius: N, lat: N, long: N) -> Self {
        let or = Vector3::new(
            radius * long.cos() * lat.cos(),
            radius * lat.sin(),
            radius * long.sin() * lat.cos(),
        );

        Vertex {
            lat,
            long,
            p: center + or,
            nc: or,
        }
    }

    fn make_triangle(va: &Self, vb: &Self, vc: &Self) -> Triangle<N> {
        Triangle {
            a: va.p,

            m_abc: Matrix3::from_cols(
                vb.p - va.p,
                vc.p - va.p,
                Vector3::cross(vb.p - va.p, vc.p - va.p).norm(),
            )
            .inv()
            .unwrap(),

            meta: Box::new(TriangleMeta {
                a: va.p,
                b: vb.p,
                c: vc.p,

                abc_nc: Matrix3::from_cols(va.nc, vb.nc, vc.nc),

                abc_uv: Matrix3::from_cols(
                    Vector3::new(va.long, va.lat, N::ZERO),
                    Vector3::new(vb.long, vb.lat, N::ZERO),
                    Vector3::new(vc.long, vc.lat, N::ZERO),
                ),
            }),
        }
    }
}

pub fn make_uv_sphere<N: Num>(
    center: Vector3<N>,
    radius: N,
    n_subdiv_lat: usize,
    n_subdiv_long: usize,
) -> TriangleList<N> {
    let mut sphere = TriangleList { triangles: vec![] };

    let lat_step = N::PI / N::from_usize(n_subdiv_lat);
    let long_step = (N::PI + N::PI) / N::from_usize(n_subdiv_long);

    for i_lat in 0..n_subdiv_lat {
        for i_long in 0..n_subdiv_long {
            let lat0 = N::PI / (N::ONE + N::ONE) + N::from_usize(i_lat) * lat_step;
            let long0 = N::from_usize(i_long) * long_step;

            let v1 = Vertex::new(center, radius, lat0, long0);
            let v2 = Vertex::new(center, radius, lat0 + lat_step, long0);
            let v3 = Vertex::new(center, radius, lat0 + lat_step, long0 + long_step);
            let v4 = Vertex::new(center, radius, lat0, long0 + long_step);

            sphere.triangles.push(Vertex::make_triangle(&v1, &v2, &v3));
            sphere.triangles.push(Vertex::make_triangle(&v1, &v3, &v4));
        }
    }

    sphere
}
