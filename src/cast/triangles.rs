use crate::math::*;

use super::*;

use std::intrinsics::likely;

pub fn cast_ray_through_triangles<'a>(
    ray: Ray,
    triangles: &'a [Triangle],
    max_d: ff32,
) -> Option<RayIntersection<'a>> {
    const EPS: ff32 = ff32(1.0e-4);

    let mut cur_d = max_d;
    let mut cur_tri: Option<&Triangle> = None;
    let mut cur_p_abc = ff32_3::ZERO;

    for tri in triangles {
        let src_abc = tri.m_abc * (ray.src - tri.a);
        let dir_abc = tri.m_abc * ray.dir1;

        if src_abc.z() < EPS || dir_abc.z() > EPS {
            continue;
        }

        let d = -src_abc.z() / dir_abc.z();

        if d >= cur_d {
            continue;
        }

        let p_abc = src_abc + dir_abc * d;

        if likely(p_abc.x() < -EPS || p_abc.y() < -EPS || p_abc.x() + p_abc.y() > ff32(1.0) + EPS) {
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

pub fn interpolate_triangle_meta<'a>(isec: &'a RayIntersection<'a>) -> InterpolatedMeta {
    let w = ff32_3::new(
        ff32(1.0) - isec.p_abc.x() - isec.p_abc.y(),
        isec.p_abc.x(),
        isec.p_abc.y(),
    );

    InterpolatedMeta {
        w,
        n1_p: (isec.tri.meta.abc_nc * w).norm(),
        p_uv: isec.tri.meta.abc_uv * w,
    }
}
