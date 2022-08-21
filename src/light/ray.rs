use crate::affine::*;
use crate::geometry::*;
use crate::linear::*;
use crate::numeric::*;

#[derive(Debug, Clone)]
pub struct Ray<N: Num> {
    pub origin: Vector3<N>,
    pub direction: Vector3<N>,
}

impl<N: Num> Geometry for Ray<N> {
    type Num = N;

    fn apply(mut self, xform: &Transform3<N>) -> Self {
        self.origin = xform.map_point(self.origin);
        self.direction = xform.map_point(self.direction);
        self
    }
}
