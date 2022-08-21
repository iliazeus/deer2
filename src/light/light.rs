use crate::numeric::*;

#[derive(Debug, Clone)]
pub struct Light<N: Num> {
    pub wavelength: N,
    pub intensity: N,
}
