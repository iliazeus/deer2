use crate::linear::*;

use super::*;

/// Vertices are ordered counter-clockwise (A -> B -> C)
pub trait Triangle: Geometry {
    type Vertex: Point<Num = Self::Num>;

    fn from_vertices(a: Self::Vertex, b: Self::Vertex, c: Self::Vertex) -> Self;
    fn into_vertices(self) -> (Self::Vertex, Self::Vertex, Self::Vertex);

    fn vertex_a(&self) -> Self::Vertex;
    fn vertex_b(&self) -> Self::Vertex;
    fn vertex_c(&self) -> Self::Vertex;

    fn normal(&self) -> Vector3<Self::Num>;
}
