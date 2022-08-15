use crate::affine::*;
use crate::geometry::*;
use crate::linear::*;
use crate::numeric::*;

#[derive(Debug, Clone)]
pub struct SurfacePoint<T: Num> {
    pub point: Vector3<T>,
    pub normal: Vector3<T>,
}

impl<T: Num> Geometry for SurfacePoint<T> {
    type Num = T;

    fn apply(mut self, xform: &Transform3<T>) -> Self {
        self.point = xform.map_point(self.point);
        self.normal = xform.map_vector(self.normal);
        self
    }
}
