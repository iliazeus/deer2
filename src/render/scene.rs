use crate::camera::*;
use crate::cast::*;
use crate::geometry::Geometry;
use crate::light::*;
use crate::light_source::*;
use crate::linear::Vector3;
use crate::material::*;
use crate::numeric::*;

pub struct Scene<N, C, M, L>
where
    N: Num,
    C: CastRay<Num = N>,
    M: Material<Num = N, Meta = C::MaterialMeta>,
    L: LightSource<Num = N>,
{
    pub castable: C,
    pub material: M,
    pub light_source: L,
    pub camera: Camera<N>,
}

impl<N, C, M, L> Scene<N, C, M, L>
where
    N: Num,
    C: CastRay<Num = N>,
    M: Material<Num = N, Meta = C::MaterialMeta>,
    L: LightSource<Num = N>,
{
    pub fn trace_pixel<R: Random<N>>(
        &self,
        screen_pixel: Vector3<N>,
        wavelength: N,
        rng: &mut R,
        reflection_count: usize,
    ) -> N {
        let mut ray = self.camera.cast_ray_through_pixel(screen_pixel);

        let mut light = Light {
            wavelength,
            intensity: N::one(),
        };

        let mut result = N::zero();

        for _ in 0..reflection_count {
            match self.castable.cast_ray(ray) {
                None => break,

                Some(SurfacePoint {
                    fwd_uv_ray,
                    uv_xform,
                    inv_uv_xform,
                    meta,
                }) => {
                    let fwd_ray = fwd_uv_ray.apply(&inv_uv_xform);
                    let bwd_ray = self.light_source.cast_ray_from(fwd_ray.origin, rng);
                    let bwd_uv_ray = bwd_ray.apply(&uv_xform);

                    if let Some(refl_light) = self
                        .material
                        .query_reflection(bwd_uv_ray, fwd_uv_ray, light, rng, &meta)
                    {
                        let exposure = self.light_source.get_exposure(fwd_ray, refl_light, rng);
                        result += N::one() / exposure;
                    }

                    match self
                        .material
                        .trace_reflection(fwd_uv_ray, light, rng, &meta)
                    {
                        None => break,
                        Some((refl_uv_ray, refl_light)) => {
                            ray = refl_uv_ray.apply(&inv_uv_xform);
                            light = refl_light;
                        }
                    }
                }
            }
        }

        result
    }
}
