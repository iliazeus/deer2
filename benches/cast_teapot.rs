#![feature(test)]

use deer2::cast::*;
use deer2::formats::stl::*;
use deer2::math::*;

use std::io::Cursor;

use rand::rngs::SmallRng;
use rand::SeedableRng;

extern crate test;
use test::{black_box, Bencher};

const UTAH_TEAPOT: &[u8] = include_bytes!("../data/stl/utah_teapot.stl");
const RESOLUTION: usize = 100;

fn cast_teapot_generic<'a, C: Castable<'a, ff32>>(b: &mut Bencher, castable: &'a C) {
    let pov = ff32_3::new(ff32(0.0), ff32(0.0), ff32(26.0));
    let screen_00 = ff32_3::new(ff32(-0.5), ff32(0.5), ff32(25.0));
    let screen_step = ff32(1.0) / ff32::from_usize(RESOLUTION);

    b.iter(|| {
        for pixel_x in 0..RESOLUTION {
            for pixel_y in 0..RESOLUTION {
                let screen_rel = ff32_3::new(
                    screen_step * ff32::from_usize(pixel_x),
                    -screen_step * ff32::from_usize(pixel_y),
                    ff32(0.0),
                );

                let screen_p = screen_00 + screen_rel;

                let ray = Ray {
                    src: pov,
                    dir1: (screen_p - pov).norm(),
                };

                let isec = castable.cast_ray(ray, ff32(2000.0));

                black_box(isec);
            }
        }
    })
}

#[bench]
fn cast_teapot_triangle_list(b: &mut Bencher) {
    let model = StlModel::read_from(&mut Cursor::new(UTAH_TEAPOT)).unwrap();
    let triangles = model.to_triangle_list();

    cast_teapot_generic(b, &triangles)
}

#[bench]
fn cast_teapot_kd_tree(b: &mut Bencher) {
    let model = StlModel::read_from(&mut Cursor::new(UTAH_TEAPOT)).unwrap();
    let triangles = model.to_triangle_list();

    let tree = BspTree::build_kd(&triangles.triangles);

    cast_teapot_generic(b, &tree)
}

#[bench]
fn cast_teapot_bsp_tree(b: &mut Bencher) {
    let model = StlModel::read_from(&mut Cursor::new(UTAH_TEAPOT)).unwrap();
    let triangles = model.to_triangle_list();

    let mut rng = SmallRng::seed_from_u64(117);
    let tree = BspTree::build_tri_randomized(&triangles.triangles, &mut rng, 1);

    cast_teapot_generic(b, &tree)
}
