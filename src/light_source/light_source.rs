use crate::light::*;
use crate::linear::*;
use crate::numeric::*;

pub trait LightSource<R: Random<Self::Num>> {
    type Num: Num;

    fn cast_ray_from(&self, origin: Vector3<Self::Num>, rng: &mut R) -> Option<Ray<Self::Num>>;

    fn get_exposure(
        &self,
        fwd_ray: Ray<Self::Num>,
        light: Light<Self::Num>,
        rng: &mut R,
    ) -> Option<Self::Num>;
}
