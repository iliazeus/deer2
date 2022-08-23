#![feature(test)]

use deer2::math::*;

extern crate test;
use test::Bencher;

fn generic_bench<N: Num, const SIZE: usize>() -> N {
    let mut accum = N::ZERO;

    for i in 0..SIZE {
        let v1 = Vector3(
            N::from_usize(i + 0),
            N::from_usize(i + 1),
            N::from_usize(i + 2),
        );
        let v2 = Vector3(
            N::from_usize(i + 3),
            N::from_usize(i + 4),
            N::from_usize(i + 5),
        );
        let v3 = Vector3(
            N::from_usize(i + 6),
            N::from_usize(i + 7),
            N::from_usize(i + 8),
        );

        let m = Matrix3(v1, v2, v3);
        accum += (m * m).det();
    }

    accum
}

#[bench]
fn bench_ff32(b: &mut Bencher) {
    b.iter(|| generic_bench::<ff32, 1_000_000>())
}

#[bench]
fn bench_f32(b: &mut Bencher) {
    b.iter(|| generic_bench::<f32, 1_000_000>())
}

#[bench]
fn bench_f64(b: &mut Bencher) {
    b.iter(|| generic_bench::<f64, 1_000_000>())
}
