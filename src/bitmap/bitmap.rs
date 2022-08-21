use super::*;

pub trait Bitmap {
    type Pixel: Pixel;

    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn get_pixel(&self, x: usize, y: usize) -> &Self::Pixel;
    fn get_mut_pixel(&mut self, x: usize, y: usize) -> &mut Self::Pixel;

    fn with_dimensions(width: usize, height: usize, fill: Self::Pixel) -> Self;

    fn from_pixels<I>(width: usize, height: usize, pixels: I) -> Self
    where
        I: Iterator<Item = Self::Pixel>;
}
