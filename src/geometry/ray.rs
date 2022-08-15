use crate::linear::*;

use super::*;

pub trait Ray: Geometry {
    fn from_source_and_direction(src: Vector3<Self::Num>, dir: Vector3<Self::Num>) -> Self;
    fn into_source_and_direction(self) -> (Vector3<Self::Num>, Vector3<Self::Num>);

    fn source(&self) -> Vector3<Self::Num>;
    fn direction(&self) -> Vector3<Self::Num>;
}
