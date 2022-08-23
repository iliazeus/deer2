#![feature(test)]

use deer2::cast::*;
use deer2::data::stl::*;
use deer2::formats::stl::*;
use deer2::math::*;

use std::io::Cursor;

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn cast_teapot_naive(b: &mut Bencher) {
    let model = StlModel::read_from(&mut Cursor::new(UTAH_TEAPOT)).unwrap();
    let triangles = model.to_triangle_list();

    const N: usize = 100;

    let pov = ff32_3::new(ff32(0.0), ff32(0.0), ff32(26.0));
    let screen_00 = ff32_3::new(ff32(-0.5), ff32(0.5), ff32(25.0));
    let screen_step = ff32(1.0) / ff32::from_usize(N);

    b.iter(|| {
        for pixel_x in 0..N {
            for pixel_y in 0..N {
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

                let isec = triangles.cast_ray(ray, ff32(2000.0));

                black_box(isec);
            }
        }
    })
}
