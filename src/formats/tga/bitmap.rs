use crate::bitmap::*;

use std::io;
use std::io::{Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt, LE};

/// Only TGA 1.0 uncompressed 24-bit images are supported.
/// Only top-to-bottom, left-to-right pixel order is supported.
pub struct TgaBitmap {
    color_map_type: ColorMapType,
    image_type: ImageType,

    color_map_first_index: u16,
    color_map_length: u16,
    color_map_entry_size: u8,

    origin_x: u16,
    origin_y: u16,
    width: u16,
    height: u16,
    pixel_depth: u8,

    alpha_depth: u8,
    is_top_to_bottom: bool,
    is_right_to_left: bool,

    id: String,
    color_map_bytes: Vec<u8>,
    pixels: Vec<u8_rgb>,
}

#[repr(u8)]
pub enum ColorMapType {
    None = 0,
    Present = 1,
}

impl From<u8> for ColorMapType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Present,
            _ => panic!("invalid color map type"),
        }
    }
}

#[repr(u8)]
pub enum ImageType {
    None = 0,
    ColorMapped = 1,
    TrueColor = 2,
    Grayscale = 3,
    RleColorMapped = 9,
    RleTrueColor = 10,
    RleGrayscale = 11,
}

impl From<u8> for ImageType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::ColorMapped,
            2 => Self::TrueColor,
            3 => Self::Grayscale,
            9 => Self::RleColorMapped,
            10 => Self::RleTrueColor,
            11 => Self::RleGrayscale,
            _ => panic!("invalid image type"),
        }
    }
}

impl Bitmap for TgaBitmap {
    type Pixel = u8_rgb;

    #[inline(always)]
    fn width(&self) -> usize {
        self.width as usize
    }

    #[inline(always)]
    fn height(&self) -> usize {
        self.height as usize
    }

    #[inline(always)]
    fn get_pixel(&self, x: usize, y: usize) -> &u8_rgb {
        &self.pixels[y * self.width() + x]
    }

    #[inline(always)]
    fn get_mut_pixel(&mut self, x: usize, y: usize) -> &mut u8_rgb {
        let width = self.width();
        &mut self.pixels[y * width + x]
    }

    fn with_dimensions(width: usize, height: usize, fill: u8_rgb) -> Self {
        TgaBitmap {
            id: String::new(),
            color_map_type: ColorMapType::None,
            image_type: ImageType::TrueColor,
            color_map_first_index: 0,
            color_map_length: 0,
            color_map_entry_size: 0,
            origin_x: 0,
            origin_y: 0,
            width: width.try_into().expect("bitmap width too big"),
            height: height.try_into().expect("image height too big"),
            pixel_depth: 8,
            alpha_depth: 0,
            is_top_to_bottom: true,
            is_right_to_left: false,
            color_map_bytes: vec![],
            pixels: vec![fill; width * height],
        }
    }
}

impl TgaBitmap {
    fn read_pixel_from<R: Read>(reader: &mut R) -> Result<u8_rgb, io::Error> {
        let r = reader.read_u8()?;
        let g = reader.read_u8()?;
        let b = reader.read_u8()?;

        Ok(u8_rgb { r, g, b })
    }

    fn write_pixel_to<W: Write>(pixel: &u8_rgb, writer: &mut W) -> Result<(), io::Error> {
        writer.write_u8(pixel.r)?;
        writer.write_u8(pixel.g)?;
        writer.write_u8(pixel.b)?;

        Ok(())
    }

    pub fn read_from<R: Read>(reader: &mut R) -> Result<Self, io::Error> {
        let id_length = reader.read_u8()?;
        let color_map_type = reader.read_u8()?;
        let image_type = reader.read_u8()?;

        let color_map_first_index = reader.read_u16::<LE>()?;
        let color_map_length = reader.read_u16::<LE>()?;
        let color_map_entry_size = reader.read_u8()?;

        let origin_x = reader.read_u16::<LE>()?;
        let origin_y = reader.read_u16::<LE>()?;
        let width = reader.read_u16::<LE>()?;
        let height = reader.read_u16::<LE>()?;

        let pixel_depth = reader.read_u8()?;

        let descriptor = reader.read_u8()?;

        let alpha_depth = descriptor & 0x0F;
        let is_right_to_left = descriptor & 0x10 != 0;
        let is_top_to_bottom = descriptor & 0x20 != 0;

        let mut id = String::with_capacity(id_length as usize);
        reader
            .by_ref()
            .take(id_length as u64)
            .read_to_string(&mut id)?;

        let color_map_size = (color_map_length as usize) * (color_map_entry_size as usize);
        let mut color_map_bytes = Vec::<u8>::with_capacity(color_map_size);
        reader
            .by_ref()
            .take(color_map_size as u64)
            .read_to_end(&mut color_map_bytes)?;

        let pixel_count = (width as usize) * (height as usize);
        let mut pixels = Vec::<u8_rgb>::with_capacity(pixel_count);
        for _ in 0..pixel_count {
            pixels.push(Self::read_pixel_from(reader)?);
        }

        Ok(TgaBitmap {
            color_map_type: color_map_type.into(),
            image_type: image_type.into(),

            color_map_first_index,
            color_map_length,
            color_map_entry_size,

            origin_x,
            origin_y,
            width,
            height,

            pixel_depth,

            alpha_depth,
            is_right_to_left,
            is_top_to_bottom,

            id,
            color_map_bytes,
            pixels,
        })
    }

    pub fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), io::Error> {
        writer.write_u8(self.id.len().try_into().unwrap())?;
        writer.write_u8(self.color_map_type as u8)?;
        writer.write_u8(self.image_type as u8)?;

        writer.write_u16::<LE>(self.color_map_first_index)?;
        writer.write_u16::<LE>(self.color_map_length)?;
        writer.write_u8(self.color_map_entry_size)?;

        writer.write_u16::<LE>(self.origin_x)?;
        writer.write_u16::<LE>(self.origin_y)?;
        writer.write_u16::<LE>(self.width)?;
        writer.write_u16::<LE>(self.height)?;

        writer.write_u8(self.pixel_depth)?;

        let descriptor = self.alpha_depth
            | (if self.is_right_to_left { 0x10 } else { 0 })
            | (if self.is_top_to_bottom { 0x20 } else { 0 });
        writer.write_u8(descriptor)?;

        writer.write_all(self.id.as_bytes())?;
        writer.write_all(&self.color_map_bytes)?;

        for pixel in self.pixels.iter() {
            Self::write_pixel_to(pixel, writer)?;
        }

        Ok(())
    }
}
