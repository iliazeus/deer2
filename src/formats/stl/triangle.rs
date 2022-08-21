use crate::cast;
use crate::math::*;

use std::io;
use std::io::{Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt, LE};

#[derive(Debug, Clone)]
pub struct StlTriangle {
    pub n: ff32_3,
    pub a: ff32_3,
    pub b: ff32_3,
    pub c: ff32_3,
    pub attr: u16,
}

impl StlTriangle {
    fn read_ff32_3_from<R: Read>(reader: &mut R) -> Result<ff32_3, io::Error> {
        let x = reader.read_f32::<LE>()?;
        let y = reader.read_f32::<LE>()?;
        let z = reader.read_f32::<LE>()?;

        Ok(ff32_3::new(ff32(x), ff32(y), ff32(z)))
    }

    fn write_ff32_3_to<W: Write>(vector: &ff32_3, writer: &mut W) -> Result<(), io::Error> {
        writer.write_f32::<LE>(vector.0 .0)?;
        writer.write_f32::<LE>(vector.1 .0)?;
        writer.write_f32::<LE>(vector.2 .0)?;

        Ok(())
    }

    pub fn read_from<R: Read>(reader: &mut R) -> Result<Self, io::Error> {
        let n = Self::read_ff32_3_from(reader)?;

        let a = Self::read_ff32_3_from(reader)?;
        let b = Self::read_ff32_3_from(reader)?;
        let c = Self::read_ff32_3_from(reader)?;

        let attr = reader.read_u16::<LE>()?;

        Ok(Self { n, a, b, c, attr })
    }

    pub fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), io::Error> {
        Self::write_ff32_3_to(&self.n, writer)?;

        Self::write_ff32_3_to(&self.a, writer)?;
        Self::write_ff32_3_to(&self.b, writer)?;
        Self::write_ff32_3_to(&self.c, writer)?;

        writer.write_u16::<LE>(self.attr)?;

        Ok(())
    }

    pub fn into_cast_triangle(self) -> cast::Triangle {
        let n = if self.n != ff32_3::zero() {
            self.n
        } else {
            ff32_3::cross(self.b - self.a, self.c - self.a)
        };

        cast::Triangle {
            a: self.a,
            ab: self.b - self.a,
            ab_abs2: (self.b - self.a).abs2(),
            ac: self.c - self.a,
            ac_abs2: (self.c - self.a).abs2(),
            n1: n / n.abs(),
            // STL does not store vertex normals
            n_a: n / n.abs(),
            n_b: n / n.abs(),
            n_c: n / n.abs(),
        }
    }
}
