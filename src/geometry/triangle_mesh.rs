use crate::linear::*;

use super::*;

pub trait TriangleMesh<'a>: 'a + Geometry {
    type Triangle: Triangle;
    type Triangles: Iterator<Item = &'a Self::Triangle>;

    fn triangles(&'a self) -> Self::Triangles;

    type Vertices: Iterator<Item = Vector3<Self::Num>>;

    fn vertices(&'a self) -> Self::Vertices;
}
