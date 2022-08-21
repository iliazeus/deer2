use crate::affine::*;
use crate::geometry::*;
use crate::linear::*;
use crate::numeric::*;

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

impl Geometry for StlTriangle {
    type Num = ff32;

    fn apply(mut self, xform: &Transform3<ff32>) -> Self {
        self.n = xform.map_vector(self.n);

        self.a = xform.map_point(self.a);
        self.b = xform.map_point(self.b);
        self.c = xform.map_point(self.c);

        self
    }
}

impl Triangle for StlTriangle {
    #[inline(always)]
    fn from_vertices(a: ff32_3, b: ff32_3, c: ff32_3) -> Self {
        let n = ff32_3::cross(b - a, c - a);
        let attr = 0u16;

        StlTriangle { n, a, b, c, attr }
    }

    #[inline(always)]
    fn into_vertices(self) -> (ff32_3, ff32_3, ff32_3) {
        (self.a, self.b, self.c)
    }

    #[inline(always)]
    fn vertex_a(&self) -> ff32_3 {
        self.a
    }

    #[inline(always)]
    fn vertex_b(&self) -> ff32_3 {
        self.b
    }

    #[inline(always)]
    fn vertex_c(&self) -> ff32_3 {
        self.c
    }

    #[inline(always)]
    fn normal(&self) -> Vector3<Self::Num> {
        if self.n != ff32_3::zero() {
            self.n
        } else {
            ff32_3::cross(self.b - self.a, self.c - self.a)
        }
    }
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
}
