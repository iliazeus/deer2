use crate::math::*;

use std::intrinsics::{likely, unlikely};

#[derive(Debug)]
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

    /// vector AB
    pub ab: ff32_3,

    /// dot(AB, AB)
    pub ab_abs2: ff32,

    /// vector AC
    pub ac: ff32_3,

    /// dot(AC, AC)
    pub ac_abs2: ff32,

    /// unit-length normal to triangle
    pub n1: ff32_3,

    /// normal in A
    pub n_a: ff32_3,

    /// normal in B
    pub n_b: ff32_3,

    /// normal in C
    pub n_c: ff32_3,
}

#[derive(Debug)]
pub struct RayIntersection<'a> {
    /// triangle
    pub tri: &'a Triangle,

    /// distance from origin along the ray
    pub d: ff32,

    /// intersection point P
    pub p: ff32_3,

    /// projection of AP on AB
    pub ap_ab: ff32,

    /// projection of AP on AC
    pub ap_ac: ff32,

    /// interpolated normal
    pub ni: ff32_3,
}

pub fn cast_ray_through_triangles<'a>(
    ray: Ray,
    triangles: &'a [Triangle],
    max_d: ff32,
) -> Option<RayIntersection<'a>> {
    let mut cur_d = max_d;
    let mut cur_tri: Option<&Triangle> = None;

    let mut cur_p = ff32_3::zero();

    let mut cur_ap_ab = ff32(0.0);
    let mut cur_ap_ac = ff32(0.0);

    for tri in triangles {
        let nd = ff32_3::dot(tri.n1, ray.dir1);

        if unlikely(nd == ff32(0.0)) {
            continue;
        }

        let d = ff32_3::dot(tri.a - ray.src, tri.n1);

        if unlikely(d < ff32(0.0)) {
            continue;
        }

        if d < cur_d {
            let p = ray.src + ray.dir1 * cur_d;
            let ap_ab = ff32_3::dot(p - tri.a, tri.ab) / tri.ab_abs2;
            let ap_ac = ff32_3::dot(p - tri.a, tri.ac) / tri.ac_abs2;

            if likely(ap_ab < ff32(0.0)) {
                continue;
            }

            if likely(ap_ac < ff32(0.0)) {
                continue;
            }

            if likely(ap_ab + ap_ac > ff32(1.0)) {
                continue;
            }

            cur_d = d;
            cur_tri = Some(tri);
            cur_p = p;
            cur_ap_ab = ap_ab;
            cur_ap_ac = ap_ac;
        }
    }

    if let Some(cur_tri) = cur_tri {
        // weighted vertex normals
        let wn_a = cur_tri.n_a * (ff32(1.0) - cur_ap_ab - cur_ap_ac);
        let wn_b = cur_tri.n_b * cur_ap_ab;
        let wn_c = cur_tri.n_c * cur_ap_ac;

        Some(RayIntersection {
            tri: cur_tri,
            d: cur_d,
            p: cur_p,
            ap_ab: cur_ap_ab,
            ap_ac: cur_ap_ac,
            ni: wn_a + wn_b + wn_c,
        })
    } else {
        None
    }
}
