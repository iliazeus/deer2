use deer2::affine::*;
use deer2::bitmap::*;
use deer2::camera::*;
use deer2::cast::*;
use deer2::data::stl::*;
use deer2::formats::stl::*;
use deer2::formats::tga::*;
use deer2::geometry::*;
use deer2::light_source::*;
use deer2::linear::*;
use deer2::material::*;
use deer2::numeric::*;
use deer2::render::*;

use std::fs::File;
use std::io::Cursor;

use rand::rngs::SmallRng;
use rand::SeedableRng;

fn get_model_size(model: &StlModel) -> ff32 {
    let mut min_coord = ff32(0.0);
    let mut max_coord = ff32(0.0);

    for t in model.triangles() {
        for v in [t.a, t.b, t.c] {
            if v.0 > max_coord {
                max_coord = v.0
            };
            if v.1 > max_coord {
                max_coord = v.1
            };
            if v.2 > max_coord {
                max_coord = v.2
            };

            if v.0 < min_coord {
                min_coord = v.0
            };
            if v.1 < min_coord {
                min_coord = v.1
            };
            if v.2 < min_coord {
                min_coord = v.2
            };
        }
    }

    max_coord - min_coord
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();

    let model = StlModel::read_from(&mut Cursor::new(UTAH_TEAPOT)).unwrap();

    let model_size = get_model_size(&model);

    let scene = Scene {
        castable: NaiveCastTriangleMesh(&model),
        material: SimplePbMaterial {
            spectrum: |_wl| ff32(1.0),
            roughness: ff32(0.5),
            shininess: ff32(0.2),
        },
        light_source: DirectionalLightSource {
            spectrum: |_wl| ff32(1.0),
            direction: ff32_3::new(ff32(1.0), ff32(-1.0), ff32(-1.0)),
        },
        camera: Camera {
            focus_point: ff32_3::new(ff32(0.0), ff32(0.0), model_size + model_size + ff32(1.0)),
            inv_screen_xform: ff32_xform3::new(
                ff32_3x3::one(),
                ff32_3::new(ff32(0.0), ff32(0.0), model_size + model_size),
            ),
        },
    };

    let renderer = Renderer {
        scene: &scene,

        width: 128,
        height: 128,
        rays_per_pixel: 1,
        reflection_count: 5,
        wavelength: ff32(1.0),
    };

    let mut rng = SmallRng::from_entropy();

    let raster = renderer.render(&mut rng);

    let bitmap = TgaBitmap::from_pixels(
        raster.width,
        raster.height,
        raster.pixels.into_iter().map(|val| {
            println!("{val}");
            let val = 1.0 / (1.0 - val.0);
            let val = (val * 255.0).clamp(0.0, 255.0) as u8;
            u8_rgb {
                r: val,
                g: val,
                b: val,
            }
        }),
    );

    let mut out_file = File::options()
        .create(true)
        .write(true)
        .open(filename)
        .unwrap();

    bitmap.write_to(&mut out_file).unwrap();
}
