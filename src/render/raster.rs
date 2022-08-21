use crate::numeric::*;

pub struct Raster<N: Num> {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<N>,
}

impl<N: Num> Raster<N> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![N::zero(); width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> N {
        self.pixels[y * self.width + x]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut N {
        &mut self.pixels[y * self.width + x]
    }
}
