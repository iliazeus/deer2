use crate::math::*;

#[derive(Debug)]
pub struct Triangle<N: Num> {
    /// vertex A
    pub a: Vector3<N>,

    /// transformation matrix into the (AB, AC, N1) space,
    /// where A, B, C are vertices, N1 is the unit normal
    pub m_abc: Matrix3<N>,

    /// indirection to fit into a cache line
    pub meta: Box<TriangleMeta<N>>,
}

#[derive(Debug)]
pub struct TriangleMeta<N: Num> {
    pub a: Vector3<N>,
    pub b: Vector3<N>,
    pub c: Vector3<N>,

    /// cols are vertex normals; length == curvature
    pub abc_nc: Matrix3<N>,

    /// cols are vertex UV
    pub abc_uv: Matrix3<N>,
}

#[derive(Debug)]
pub struct InterpolatedMeta<N: Num> {
    /// weights of vertices for linear interpolation
    pub w: Vector3<N>,

    /// interpolated normal
    pub n1_p: Vector3<N>,

    /// UV coords of the intersection point
    pub p_uv: Vector3<N>,
}
