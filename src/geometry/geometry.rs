use crate::affine::*;
use crate::numeric::*;

use std::fmt::Debug;

pub trait Geometry: Debug + Clone {
    type Num: Num;

    fn apply(&mut self, xform: &Transform3<Self::Num>);
}
