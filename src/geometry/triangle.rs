use crate::linear::*;

use super::*;

/// Vertices are ordered counter-clockwise (A -> B -> C)
pub trait Triangle: Geometry {
    fn from_vertices(a: Vector3<Self::Num>, b: Vector3<Self::Num>, c: Vector3<Self::Num>) -> Self;
    fn into_vertices(self) -> (Vector3<Self::Num>, Vector3<Self::Num>, Vector3<Self::Num>);

    fn vertex_a(&self) -> Vector3<Self::Num>;
    fn vertex_b(&self) -> Vector3<Self::Num>;
    fn vertex_c(&self) -> Vector3<Self::Num>;

    fn normal(&self) -> Vector3<Self::Num>;
}
