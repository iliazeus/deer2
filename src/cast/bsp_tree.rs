use std::intrinsics::likely;

use crate::math::*;

use itertools::partition;
use rand::prelude::*;

use super::*;

pub struct BspTree<'a, N: Num> {
    root: Option<Box<Node<'a, N>>>,
}

struct Node<'a, N: Num> {
    tris: Vec<&'a Triangle<N>>,
    neg: Option<Box<Node<'a, N>>>,
    pos: Option<Box<Node<'a, N>>>,
    origin: Vector3<N>,
    mat: Matrix3<N>,
}

impl<'a, N: Num> Node<'a, N> {
    fn partition(
        mut slice: &mut [&'a Triangle<N>],
        origin: Vector3<N>,
        mat: Matrix3<N>,
    ) -> (usize, usize) {
        // TODO: optimize

        let i_neg = partition(&mut slice[..], |tri| {
            let a = mat * (tri.meta.a - origin);
            let b = mat * (tri.meta.b - origin);
            let c = mat * (tri.meta.c - origin);

            a.z() < -N::EPS && b.z() < -N::EPS && c.z() < -N::EPS
        });

        slice = &mut slice[i_neg..];

        let i_pos = i_neg
            + partition(slice, |tri| {
                let a = mat * (tri.meta.a - origin);
                let b = mat * (tri.meta.b - origin);
                let c = mat * (tri.meta.c - origin);

                !(a.z() > N::EPS && b.z() > N::EPS && c.z() > N::EPS)
            });

        (i_neg, i_pos)
    }
}

impl<'a, N: Num> Node<'a, N> {
    fn build_tri(slice: &mut [&'a Triangle<N>]) -> (usize, Option<Box<Self>>) {
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

impl<'a, N: Num> BspTree<'a, N> {
    pub fn build_tri_randomized<RNG: Rng>(
        triangles: &'a [Triangle<N>],
        rng: &mut RNG,
        n_retries: usize,
    ) -> Self {
        let mut triangles: Vec<&'a Triangle<N>> = triangles.iter().collect();

        let mut min_height = triangles.len() + 1;
        let mut best_root: Option<Box<Node<'a, N>>> = None;

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

impl<'a, N: Num> Node<'a, N> {
    fn get_kd_mat(axis: usize) -> Matrix3<N> {
        let _0 = N::ZERO;
        let _1 = N::ONE;

        match axis % 3 {
            0 => Matrix3::from_rows(Vector3::EY, Vector3::EZ, Vector3::EX),
            1 => Matrix3::from_rows(Vector3::EZ, Vector3::EX, Vector3::EY),
            2 => Matrix3::from_rows(Vector3::EX, Vector3::EY, Vector3::EZ),

            _ => unreachable!(),
        }
    }

    fn get_bounds(triangles: &[&'a Triangle<N>]) -> (Vector3<N>, Vector3<N>) {
        let mut min_coords = triangles[0].a;
        let mut max_coords = triangles[0].a;

        for tri in triangles {
            min_coords = Vector3::min_coords(min_coords, tri.meta.a);
            min_coords = Vector3::min_coords(min_coords, tri.meta.b);
            min_coords = Vector3::min_coords(min_coords, tri.meta.c);

            max_coords = Vector3::max_coords(max_coords, tri.meta.a);
            max_coords = Vector3::max_coords(max_coords, tri.meta.b);
            max_coords = Vector3::max_coords(max_coords, tri.meta.c);
        }

        (min_coords, max_coords)
    }

    fn build_kd(slice: &mut [&'a Triangle<N>], axis: usize) -> Option<Box<Self>> {
        if slice.is_empty() {
            return None;
        }

        let (min_coords, max_coords) = Self::get_bounds(slice);
        let origin = (min_coords + max_coords) / (N::ONE + N::ONE);
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

impl<'a, N: Num> BspTree<'a, N> {
    pub fn build_kd(triangles: &'a [Triangle<N>]) -> Self {
        let mut triangles: Vec<&'a Triangle<N>> = triangles.iter().collect();

        Self {
            root: Node::build_kd(&mut triangles, 0),
        }
    }
}

impl<'a, N: Num> Node<'a, N> {
    fn cast_through_own(&'a self, ray: Ray<N>, max_d: N) -> Option<RayIntersection<'a, N>> {
        let mut cur_d = max_d;
        let mut cur_tri: Option<&Triangle<N>> = None;
        let mut cur_p_abc = Vector3::ZERO;

        for tri in self.tris.iter() {
            let src_abc = tri.m_abc * (ray.src - tri.a);
            let dir_abc = tri.m_abc * ray.dir1;

            if src_abc.z() < N::EPS || dir_abc.z() > N::EPS {
                continue;
            }

            let d = -src_abc.z() / dir_abc.z();

            if d >= cur_d {
                continue;
            }

            let p_abc = src_abc + dir_abc * d;

            if likely(
                p_abc.x() < -N::EPS
                    || p_abc.y() < -N::EPS
                    || p_abc.x() + p_abc.y() > N::ONE + N::EPS,
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
        isec1: Option<RayIntersection<'a, N>>,
        isec2: Option<RayIntersection<'a, N>>,
    ) -> Option<RayIntersection<'a, N>> {
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
        isec1: Option<RayIntersection<'a, N>>,
        isec2: Option<RayIntersection<'a, N>>,
        isec3: Option<RayIntersection<'a, N>>,
    ) -> Option<RayIntersection<'a, N>> {
        Self::choose_from_2(isec1, Self::choose_from_2(isec2, isec3))
    }
}

impl<'a, N: Num> Castable<'a, N> for Node<'a, N> {
    fn cast_ray(&'a self, ray: Ray<N>, max_d: N) -> Option<RayIntersection<'a, N>> {
        let src_bs = self.mat * (ray.src - self.origin);
        let dir_bs = self.mat * ray.dir1;

        if src_bs.z() < N::ZERO && dir_bs.z() < N::ZERO {
            return Self::choose_from_2(
                self.cast_through_own(ray, max_d),
                self.neg.as_ref().and_then(|n| n.cast_ray(ray, max_d)),
            );
        }

        if src_bs.z() > N::ZERO && dir_bs.z() > N::ZERO {
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

impl<'a, N: Num> Castable<'a, N> for BspTree<'a, N> {
    fn cast_ray(&'a self, ray: Ray<N>, max_d: N) -> Option<RayIntersection<'a, N>> {
        self.root.as_ref().and_then(|n| n.cast_ray(ray, max_d))
    }
}
