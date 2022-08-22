use crate::math::*;

use std::intrinsics::likely;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    /// ray source
    pub src: ff32_3,

    /// unit-length ray direction
    pub dir1: ff32_3,
}

#[derive(Debug)]
pub struct Triangle {
    /// vertex A
    pub a: ff32_3,

    /// transformation matrix into the (AB, AC, N1) space,
    /// where A, B, C are vertices, N1 is the unit normal
    pub m_abc: ff32_3x3,

    /// indirection to fit in a cache line
    pub meta: Box<TriangleMeta>,
}

#[derive(Debug)]
pub struct TriangleMeta {
    /// cols are vertex normals; length == curvature
    pub abc_nc: ff32_3x3,

    /// cols are vertex UV
    pub abc_uv: ff32_3x3,
}

#[derive(Debug)]
pub struct RayIntersection<'a> {
    /// triangle
    pub tri: &'a Triangle,

    /// distance from origin along the ray
    pub d: ff32,

    /// intersection point in (AB, AC, N1) space
    pub p_abc: ff32_3,
}

#[derive(Debug)]
pub struct InterpolatedMeta {
    /// weights of vertices for linear interpolation
    pub w: ff32_3,

    /// interpolated normal
    pub n1_p: ff32_3,

    /// UV coords of the intersection point
    pub p_uv: ff32_3,
}

pub fn cast_ray_through_triangles<'a>(
    ray: Ray,
    triangles: &'a [Triangle],
    max_d: ff32,
) -> Option<RayIntersection<'a>> {
    const EPS: ff32 = ff32(1.0e-4);

    let mut cur_d = max_d;
    let mut cur_tri: Option<&Triangle> = None;
    let mut cur_p_abc = ff32_3::zero();

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

        if likely(
            p_abc.x() < -EPS || p_abc.y() < -EPS || p_abc.x() + p_abc.y() > ff32(1.0) + EPS,
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
