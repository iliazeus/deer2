use crate::numeric::*;

pub struct Raster<N: Num> {
    pub width: usize,
    pub height: usize,
    pub planes: Vec<RasterPlane<N>>,
}

pub struct RasterPlane<N: Num> {
    pub wavelength: N,
    pub pixels: Vec<N>,
}

impl<N: Num> Raster<N> {
    pub fn new(width: usize, height: usize, plane_wavelengths: &[N]) -> Self {
        Self {
            width,
            height,
            planes: plane_wavelengths
                .into_iter()
                .map(|wl| RasterPlane::new(width, height, *wl))
                .collect(),
        }
    }
}

impl<N: Num> RasterPlane<N> {
    fn new(width: usize, height: usize, wavelength: N) -> Self {
        Self {
            wavelength,
            pixels: vec![N::zero(); width * height],
        }
    }
}
