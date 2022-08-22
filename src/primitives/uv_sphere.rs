use crate::cast::*;
use crate::math::*;

#[derive(Debug)]
pub struct UvSphere {
    pub triangles: Vec<Triangle>,
}

struct Vertex {
    lat: ff32,
    long: ff32,
    p: ff32_3,
    nc: ff32_3,
}

impl UvSphere {
    fn vertex_at(center: ff32_3, radius: ff32, lat: ff32, long: ff32) -> Vertex {
        let or = ff32_3::new(
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

    fn push_triangle(&mut self, va: &Vertex, vb: &Vertex, vc: &Vertex) {
        self.triangles.push(Triangle {
            a: va.p,

            m_abc: ff32_3x3::from_cols(
                vb.p - va.p,
                vc.p - va.p,
                ff32_3::cross(vb.p - va.p, vc.p - va.p).norm(),
            )
            .inv()
            .unwrap(),

            meta: Box::new(TriangleMeta {
                a: va.p,
                b: vb.p,
                c: vc.p,

                abc_nc: ff32_3x3::from_cols(va.nc, vb.nc, vc.nc),

                abc_uv: ff32_3x3::from_cols(
                    ff32_3::new(va.long, va.lat, ff32(0.0)),
                    ff32_3::new(vb.long, vb.lat, ff32(0.0)),
                    ff32_3::new(vc.long, vc.lat, ff32(0.0)),
                ),
            }),
        });
    }

    pub fn new(center: ff32_3, radius: ff32, n_subdiv_lat: usize, n_subdiv_long: usize) -> Self {
        let mut sphere = UvSphere { triangles: vec![] };

        let lat_step = ff32(std::f32::consts::PI / (n_subdiv_lat as f32));
        let long_step = ff32(2.0 * std::f32::consts::PI / (n_subdiv_long as f32));

        for i_lat in 0..n_subdiv_lat {
            for i_long in 0..n_subdiv_long {
                let lat0 = ff32(-std::f32::consts::FRAC_PI_2) + ff32::from_usize(i_lat) * lat_step;
                let long0 = ff32::from_usize(i_long) * long_step;

                let v1 = UvSphere::vertex_at(center, radius, lat0, long0);
                let v2 = UvSphere::vertex_at(center, radius, lat0 + lat_step, long0);
                let v3 = UvSphere::vertex_at(center, radius, lat0 + lat_step, long0 + long_step);
                let v4 = UvSphere::vertex_at(center, radius, lat0, long0 + long_step);

                sphere.push_triangle(&v1, &v2, &v3);
                sphere.push_triangle(&v1, &v3, &v4);
            }
        }

        sphere
    }
}
