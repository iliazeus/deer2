use crate::affine::*;
use crate::numeric::*;

use std::fmt::Debug;

pub trait Geometry: Debug + Clone {
    type Num: Num;

    fn apply(self, xform: &Transform3<Self::Num>) -> Self;
}
