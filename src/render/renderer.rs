use crate::cast::*;
use crate::light_source::*;
use crate::linear::*;
use crate::material::*;
use crate::numeric::*;

use super::*;

pub struct Renderer<'a, N, C, M, L>
where
    N: Num,
    C: CastRay<Num = N>,
    M: Material<Num = N, Meta = C::MaterialMeta>,
    L: LightSource<Num = N>,
{
    pub scene: &'a Scene<N, C, M, L>,

    pub width: usize,
    pub height: usize,
    pub rays_per_pixel: usize,
    pub reflection_count: usize,
    pub wavelength: N,
}

impl<'a, N, C, M, L> Renderer<'a, N, C, M, L>
where
    N: Num,
    C: CastRay<Num = N>,
    M: Material<Num = N, Meta = C::MaterialMeta>,
    L: LightSource<Num = N>,
{
    pub fn render<R: Random<N>>(&self, rng: &mut R) -> Raster<N> {
        let _2 = N::from_usize(2);
        let _1 = N::one();
        let _0 = N::zero();

        let width = N::from_usize(self.width);
        let height = N::from_usize(self.height);

        let pass_weight = _1 / N::from_usize(self.rays_per_pixel);

        let mut raster = Raster::<N>::new(self.width, self.height);

        for pass_index in 0..self.rays_per_pixel {
            println!("progress: {}/{}", pass_index, self.rays_per_pixel);

            for pixel_y in 0..self.height {
                for pixel_x in 0..self.width {
                    // println!("{pixel_x} {pixel_y}");

                    let screen_pixel = Vector3(
                        (N::from_usize(pixel_x) / width) * _2 - _1,
                        (N::from_usize(pixel_y) / height) * _2 - _1,
                        _0,
                    );

                    let value = self.scene.trace_pixel(
                        screen_pixel,
                        self.wavelength,
                        rng,
                        self.reflection_count,
                    );

                    *raster.get_mut(pixel_x, pixel_y) += value * pass_weight;
                }
            }
        }

        raster
    }
}
