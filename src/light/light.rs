use crate::numeric::*;

#[derive(Debug, Clone, Copy)]
pub struct Light<N: Num> {
    pub wavelength: N,
    pub intensity: N,
}
