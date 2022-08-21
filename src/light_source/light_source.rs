use crate::light::*;
use crate::linear::*;
use crate::numeric::*;

pub trait LightSource {
    type Num: Num;

    fn cast_ray_from<R: Random<Self::Num>>(
        &self,
        origin: Vector3<Self::Num>,
        rng: &mut R,
    ) -> Ray<Self::Num>;

    fn get_exposure<R: Random<Self::Num>>(
        &self,
        fwd_ray: Ray<Self::Num>,
        light: Light<Self::Num>,
        rng: &mut R,
    ) -> Self::Num;
}
