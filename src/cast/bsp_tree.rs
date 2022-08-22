use std::intrinsics::{likely, unlikely};

use crate::math::*;

use itertools::partition;
use rand::prelude::*;

use super::*;

pub struct BspTree<'a> {
    root: Option<Box<Node<'a>>>,
}

struct Node<'a> {
    tris: Vec<&'a Triangle>,
    neg: Option<Box<Node<'a>>>,
    pos: Option<Box<Node<'a>>>,
    origin: ff32_3,
    mat: ff32_3x3,
}

impl<'a> Node<'a> {
    fn partition(mut slice: &mut [&'a Triangle], origin: ff32_3, mat: ff32_3x3) -> (usize, usize) {
        // TODO: optimize

        const EPS: ff32 = ff32(1.0e-4);

        let i_neg = partition(&mut slice[..], |tri| {
            let a = mat * (tri.meta.a - origin);
            let b = mat * (tri.meta.b - origin);
            let c = mat * (tri.meta.c - origin);

            a.z() < -EPS && b.z() < -EPS && c.z() < -EPS
        });

        slice = &mut slice[i_neg..];

        let i_pos = i_neg
            + partition(slice, |tri| {
                let a = mat * (tri.meta.a - origin);
                let b = mat * (tri.meta.b - origin);
                let c = mat * (tri.meta.c - origin);

                !(a.z() > EPS && b.z() > EPS && c.z() > EPS)
            });

        (i_neg, i_pos)
    }
}

impl<'a> Node<'a> {
    fn build_tri(slice: &mut [&'a Triangle]) -> (usize, Option<Box<Self>>) {
        if slice.is_empty() {
            return (0, None);
        }

        if slice.len() == 1 {
            let tri = slice[0];

            return (
                1,
                Some(Box::new(Self {
                    tris: Vec::from(slice),
                    neg: None,
                    pos: None,
                    origin: tri.a,
                    mat: tri.m_abc,
                })),
            );
        }

        let tri = slice[0];

        let (i_neg, i_pos) = Self::partition(slice, tri.a, tri.m_abc);

        let (h_neg, neg) = Self::build_tri(&mut slice[..i_neg]);
        let (h_pos, pos) = Self::build_tri(&mut slice[i_pos..]);

        (
            1 + usize::max(h_neg, h_pos),
            Some(Box::new(Self {
                tris: Vec::from(&slice[i_neg..i_pos]),
                neg,
                pos,
                origin: tri.a,
                mat: tri.m_abc,
            })),
        )
    }
}

impl<'a> BspTree<'a> {
    pub fn build_tri_randomized<RNG: Rng>(
        triangles: &'a [Triangle],
        rng: &mut RNG,
        n_retries: usize,
    ) -> Self {
        let mut triangles: Vec<&'a Triangle> = triangles.iter().collect();

        let mut min_height = triangles.len() + 1;
        let mut best_root: Option<Box<Node>> = None;

        for _i in 0..n_retries {
            triangles.shuffle(rng);
            let root = Node::build_tri(&mut triangles);

            if let (height, Some(root)) = root && height < min_height {
                min_height = height;
                best_root = Some(root);
            }
        }

        Self { root: best_root }
    }
}

impl<'a> Node<'a> {
    fn get_kd_mat(axis: usize) -> ff32_3x3 {
        const _0: ff32 = ff32(0.0);
        const _1: ff32 = ff32(1.0);

        match axis % 3 {
            0 => ff32_3x3::from_rows(
                ff32_3::new(_0, _1, _0),
                ff32_3::new(_0, _0, _1),
                ff32_3::new(_1, _0, _0),
            ),

            1 => ff32_3x3::from_rows(
                ff32_3::new(_0, _0, _1),
                ff32_3::new(_1, _0, _0),
                ff32_3::new(_0, _1, _0),
            ),

            2 => ff32_3x3::from_rows(
                ff32_3::new(_1, _0, _0),
                ff32_3::new(_0, _1, _0),
                ff32_3::new(_0, _0, _1),
            ),

            _ => unreachable!(),
        }
    }

    fn get_bounds(triangles: &[&'a Triangle]) -> (ff32_3, ff32_3) {
        let mut min_coords = triangles[0].a;
        let mut max_coords = triangles[0].a;

        for tri in triangles {
            min_coords = ff32_3::min_coords(min_coords, tri.meta.a);
            min_coords = ff32_3::min_coords(min_coords, tri.meta.b);
            min_coords = ff32_3::min_coords(min_coords, tri.meta.c);

            max_coords = ff32_3::max_coords(max_coords, tri.meta.a);
            max_coords = ff32_3::max_coords(max_coords, tri.meta.b);
            max_coords = ff32_3::max_coords(max_coords, tri.meta.c);
        }

        (min_coords, max_coords)
    }

    fn build_kd(slice: &mut [&'a Triangle], axis: usize) -> Option<Box<Self>> {
        if slice.is_empty() {
            return None;
        }

        let (min_coords, max_coords) = Self::get_bounds(slice);
        let origin = (min_coords + max_coords) / ff32(2.0);
        let mat = Self::get_kd_mat(axis);

        let (i_neg, i_pos) = Self::partition(slice, origin, mat);

        let neg = Self::build_kd(&mut slice[..i_neg], axis + 1);
        let pos = Self::build_kd(&mut slice[i_pos..], axis + 1);

        Some(Box::new(Self {
            tris: Vec::from(&slice[i_neg..i_pos]),
            mat,
            origin,
            neg,
            pos,
        }))
    }
}

impl<'a> BspTree<'a> {
    pub fn build_kd(triangles: &'a [Triangle]) -> Self {
        let mut triangles: Vec<&'a Triangle> = triangles.iter().collect();

        Self {
            root: Node::build_kd(&mut triangles, 0),
        }
    }
}

impl<'a> Node<'a> {
    fn cast_through_own(&'a self, ray: Ray, max_d: ff32) -> Option<RayIntersection<'a>> {
        const EPS: ff32 = ff32(1.0e-4);

        let mut cur_d = max_d;
        let mut cur_tri: Option<&Triangle> = None;
        let mut cur_p_abc = ff32_3::zero();

        for tri in self.tris.iter() {
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

    #[inline(always)]
    fn choose_from_2(
        isec1: Option<RayIntersection<'a>>,
        isec2: Option<RayIntersection<'a>>,
    ) -> Option<RayIntersection<'a>> {
        match (isec1, isec2) {
            (isec1, None) => isec1,
            (None, isec2) => isec2,

            (Some(isec1), Some(isec2)) => {
                if isec1.d <= isec2.d {
                    Some(isec1)
                } else {
                    Some(isec2)
                }
            }
        }
    }

    #[inline(always)]
    fn choose_from_3(
        isec1: Option<RayIntersection<'a>>,
        isec2: Option<RayIntersection<'a>>,
        isec3: Option<RayIntersection<'a>>,
    ) -> Option<RayIntersection<'a>> {
        Self::choose_from_2(isec1, Self::choose_from_2(isec2, isec3))
    }

    fn cast_ray(&'a self, ray: Ray, max_d: ff32) -> Option<RayIntersection<'a>> {
        const EPS: ff32 = ff32(1.0e-4);

        let src_bs = self.mat * (ray.src - self.origin);
        let dir_bs = self.mat * ray.dir1;

        if src_bs.z() < ff32(0.0) && dir_bs.z() < ff32(0.0) {
            return Self::choose_from_2(
                self.cast_through_own(ray, max_d),
                self.neg.as_ref().and_then(|n| n.cast_ray(ray, max_d)),
            );
        }

        if src_bs.z() > ff32(0.0) && dir_bs.z() > ff32(0.0) {
            return Self::choose_from_2(
                self.cast_through_own(ray, max_d),
                self.pos.as_ref().and_then(|n| n.cast_ray(ray, max_d)),
            );
        }

        Self::choose_from_3(
            self.cast_through_own(ray, max_d),
            self.neg.as_ref().and_then(|n| n.cast_ray(ray, max_d)),
            self.pos.as_ref().and_then(|n| n.cast_ray(ray, max_d)),
        )
    }
}

impl<'a> BspTree<'a> {
    pub fn cast_ray(&'a self, ray: Ray, max_d: ff32) -> Option<RayIntersection<'a>> {
        self.root.as_ref().and_then(|n| n.cast_ray(ray, max_d))
    }
}
