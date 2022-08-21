use crate::cast::*;
use crate::math::*;

pub struct UvSphere {
    pub triangles: Vec<Triangle>,
}

struct Vertex {
    lat: ff32,
    long: ff32,
    p: ff32_3,
    n: ff32_3,
}

impl UvSphere {
    fn vertex_at(center: ff32_3, radius: ff32, lat: ff32, long: ff32) -> Vertex {
        let or = ff32_3::new(
            radius * long.cos() * lat.sin(),
            radius * lat.cos(),
            -radius * long.sin() * lat.sin(),
        );

        Vertex {
            lat,
            long,
            p: center + or,
            n: or,
        }
    }

    fn push_triangle(&mut self, va: &Vertex, vb: &Vertex, vc: &Vertex) {
        self.triangles.push(Triangle {
            a: va.p,

            ab: vb.p - va.p,
            ab_abs2: (vb.p - va.p).abs2(),

            ac: vc.p - va.p,
            ac_abs2: (vc.p - va.p).abs2(),

            n1: ff32_3::cross(vb.p - va.p, vc.p - va.p).norm(),

            meta: Box::new(TriangleMeta {
                n_a: va.n,
                n_b: vb.n,
                n_c: vc.n,

                a_u: va.long,
                a_v: va.lat,

                b_u: vb.long,
                b_v: vb.lat,

                c_u: vb.long,
                c_v: vb.lat,
            }),
        });
    }

    pub fn new(center: ff32_3, radius: ff32, n_subdiv_lat: usize, n_subdiv_long: usize) -> Self {
        let mut sphere = UvSphere { triangles: vec![] };

        let lat_step = ff32(2.0 * std::f32::consts::PI / (n_subdiv_lat as f32));
        let long_step = ff32(std::f32::consts::PI / (n_subdiv_long as f32));

        for i_lat in 0..n_subdiv_lat {
            for i_long in 0..n_subdiv_long {
                let lat0 = -ff32::pi() + ff32::from_usize(i_lat) * lat_step;
                let long0 = ff32::from_usize(i_long) * long_step;

                let v1 = UvSphere::vertex_at(center, radius, lat0, long0);
                let v2 = UvSphere::vertex_at(center, radius, lat0, long0 + long_step);
                let v3 = UvSphere::vertex_at(center, radius, lat0 + lat_step, long0 + long_step);
                let v4 = UvSphere::vertex_at(center, radius, lat0 + lat_step, long0);

                sphere.push_triangle(&v1, &v2, &v3);
                sphere.push_triangle(&v2, &v3, &v4);
            }
        }

        sphere
    }
}
