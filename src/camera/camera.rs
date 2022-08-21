use crate::affine::*;
use crate::geometry::*;
use crate::light::*;
use crate::linear::*;
use crate::numeric::*;

#[derive(Debug, Clone)]
pub struct Camera<N: Num> {
    pub pixel_width: usize,
    pub pixel_height: usize,

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
    pub fn cast_ray_through_pixel(&self, pixel_x: usize, pixel_y: usize) -> Ray<N> {
        let pixel_x = N::from_usize(pixel_x);
        let pixel_y = N::from_usize(pixel_y);

        let pixel_width = N::from_usize(self.pixel_width);
        let pixel_height = N::from_usize(self.pixel_height);

        let screen_pixel = Vector3(
            (pixel_x / pixel_width) * N::from_usize(2) - N::one(),
            (pixel_y / pixel_height) * N::from_usize(2) - N::one(),
            N::zero(),
        );

        let world_pixel = self.inv_screen_xform.map_point(screen_pixel);

        Ray {
            origin: self.focus_point.clone(),
            direction: world_pixel,
        }
    }
}
