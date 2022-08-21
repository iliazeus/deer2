use crate::affine::*;
use crate::geometry::*;
use crate::light::*;
use crate::linear::*;
use crate::numeric::*;

#[derive(Debug, Clone)]
pub struct Camera<N: Num> {
    pub focus_point: Vector3<N>,
    pub inv_screen_xform: Transform3<N>,
}

impl<N: Num> Geometry for Camera<N> {
    type Num = N;

    fn apply(mut self, xform: &Transform3<N>) -> Self {
        self.focus_point = xform.map_point(self.focus_point);
        self.inv_screen_xform = self.inv_screen_xform.chain(xform);
        self
    }
}

impl<N: Num> Camera<N> {
    pub fn cast_ray_through_pixel(&self, screen_pixel: Vector3<N>) -> Ray<N> {
        let world_pixel = self.inv_screen_xform.map_point(screen_pixel);

        Ray {
            origin: self.focus_point,
            direction: world_pixel,
        }
    }
}
