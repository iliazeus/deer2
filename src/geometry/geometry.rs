use crate::linear::*;
use crate::numeric::*;

use std::fmt::Debug;

pub trait Geometry: Debug {
    type Num: Num;
}

impl<T: Num> Geometry for Vector3<T> {
    type Num = T;
}
