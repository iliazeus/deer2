use deer2::cast::*;
use deer2::formats::stl::*;
use deer2::formats::tga::*;
use deer2::math::*;

use rand::rngs::SmallRng;
use rand::SeedableRng;

use std::fs::File;
use std::io::{BufReader, BufWriter};

fn main() {
    // let in_filename = std::env::args().nth(1).unwrap();
    // let out_filename = std::env::args().nth(2).unwrap();

    let in_filename = "./src/data/stl/stanford_bunny.stl";
    let out_filename = "./stanford_bunny.tga";

    // let in_filename = "./src/data/stl/utah_teapot.stl";
    // let out_filename = "./utah_teapot.tga";

    let in_file = File::options().read(true).open(in_filename).unwrap();
    let mut in_file = BufReader::with_capacity(8 * 1024 * 1024, in_file);

    let out_file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(out_filename)
        .unwrap();
    let mut out_file = BufWriter::with_capacity(8 * 1024 * 1024, out_file);

    let model = StlModel::read_from(&mut in_file).unwrap();
    let triangles = model.to_triangle_list();

    let mut rng = SmallRng::seed_from_u64(117);
    let bsp_tree = BspTree::build_tri_randomized(&triangles.triangles, &mut rng, 16);
    // let bsp_tree = BspTree::build_kd(&triangles.triangles);

    let mut bitmap = TgaBitmap::with_dimensions(512, 512, u8_rgb(0, 0, 0));
    // let mut bitmap = TgaBitmap::with_dimensions(64, 64, u8_rgb(0, 0, 0));

    // // utah teapot settings
    // let pov = ff32_3::new(ff32(0.0), ff32(0.0), ff32(26.0));
    // let screen_00 = ff32_3::new(ff32(-0.5), ff32(0.5), ff32(25.0));
    // let screen_step = ff32(1.0) / ff32::from_usize(bitmap.height());

    // stanford bunny settings
    let pov = ff32_3::new(ff32(0.0), ff32(0.0), ff32(306.0));
    let screen_00 = ff32_3::new(ff32(-0.5), ff32(0.5), ff32(305.0));
    let screen_step = ff32(1.0) / ff32::from_usize(bitmap.height());

    let light_dir1 = ff32_3::new(ff32(-1.0), ff32(1.0), ff32(1.0)).norm();

    for pixel_y in 0..bitmap.height() {
        for pixel_x in 0..bitmap.width() {
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

            // let isec = cast_ray_through_triangles(ray, &triangles, ff32(2000.0));
            let isec = bsp_tree.cast_ray(ray, ff32(2000.0));
            if let Some(isec) = isec {
                // *bitmap.get_mut(pixel_x, pixel_y) = u8_rgb(255, 0, 0);

                // let isec_meta = interpolate_triangle_meta(&isec);
                // *bitmap.get_mut(pixel_x, pixel_y) = u8_rgb(
                //     (isec_meta.n1_p.x() * ff32(127.5) + ff32(127.5)).0 as u8,
                //     (isec_meta.n1_p.y() * ff32(127.5) + ff32(127.5)).0 as u8,
                //     0,
                // );

                // *bitmap.get_mut(pixel_x, pixel_y) = u8_rgb(
                //     ((isec.d - ff32(1.0)) * ff32(100.0)).0 as u8,
                //     ((isec.d - ff32(1.0)) * ff32(100.0)).0 as u8,
                //     ((isec.d - ff32(1.0)) * ff32(100.0)).0 as u8,
                // );

                let isec_meta = isec.interpolate_meta();
                let light_dot = ff32_3::dot(light_dir1, isec_meta.n1_p);

                let mut light = ff32(0.2);
                // let mut light = ff32(0.8) * ff32(light_dot.0.clamp(0.0, 1.0));

                if light_dot > ff32(0.0) {
                    let light_ray = Ray {
                        src: ray.src + ray.dir1 * isec.d,
                        dir1: light_dir1,
                    };

                    // let light_isec = triangles.cast_ray(light_ray, ff32(1000.0));
                    let light_isec = bsp_tree.cast_ray(light_ray, ff32(2000.0));
                    if light_isec.is_none() {
                        light += ff32(0.8 * light_dot.0.clamp(0.0, 1.0))
                    }
                }

                let val = (light * ff32(255.0)).0 as u8;
                *bitmap.get_mut(pixel_x, pixel_y) = u8_rgb(val, val, val);
            }
        }
    }

    bitmap.write_to(&mut out_file).unwrap();
}
