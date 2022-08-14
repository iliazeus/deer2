use crate::geometry::*;
use crate::linear::*;
use crate::numeric::Zero;

use std::io;
use std::io::{Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt, LE};

#[derive(Debug, Clone)]
pub struct StlTriangle {
    pub n: f32_3,
    pub a: f32_3,
    pub b: f32_3,
    pub c: f32_3,
    pub attr: u16,
}

impl Geometry for StlTriangle {
    type Num = f32;
}

impl Triangle for StlTriangle {
    type Vertex = f32_3;

    #[inline(always)]
    fn from_vertices(a: Self::Vertex, b: Self::Vertex, c: Self::Vertex) -> Self {
        let n = f32_3::cross(&(b.clone() - &a), &(c.clone() - &a));
        let attr = 0u16;

        StlTriangle { n, a, b, c, attr }
    }

    #[inline(always)]
    fn into_vertices(self) -> (Self::Vertex, Self::Vertex, Self::Vertex) {
        (self.a, self.b, self.c)
    }

    #[inline(always)]
    fn vertex_a(&self) -> Self::Vertex {
        self.a.clone()
    }

    #[inline(always)]
    fn vertex_b(&self) -> Self::Vertex {
        self.b.clone()
    }

    #[inline(always)]
    fn vertex_c(&self) -> Self::Vertex {
        self.c.clone()
    }

    #[inline(always)]
    fn normal(&self) -> Vector3<Self::Num> {
        if self.n != f32_3::zero() {
            self.n.clone()
        } else {
            f32_3::cross(&(self.b.clone() - &self.a), &(self.c.clone() - &self.a))
        }
    }
}

impl StlTriangle {
    fn read_f32_3_from<R: Read>(reader: &mut R) -> Result<f32_3, io::Error> {
        let x = reader.read_f32::<LE>()?;
        let y = reader.read_f32::<LE>()?;
        let z = reader.read_f32::<LE>()?;

        Ok(f32_3::new(x, y, z))
    }

    fn write_f32_3_to<W: Write>(vector: &f32_3, writer: &mut W) -> Result<(), io::Error> {
        writer.write_f32::<LE>(vector.0)?;
        writer.write_f32::<LE>(vector.1)?;
        writer.write_f32::<LE>(vector.2)?;

        Ok(())
    }

    pub fn read_from<R: Read>(reader: &mut R) -> Result<Self, io::Error> {
        let n = Self::read_f32_3_from(reader)?;

        let a = Self::read_f32_3_from(reader)?;
        let b = Self::read_f32_3_from(reader)?;
        let c = Self::read_f32_3_from(reader)?;

        let attr = reader.read_u16::<LE>()?;

        Ok(Self { n, a, b, c, attr })
    }

    pub fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), io::Error> {
        Self::write_f32_3_to(&self.n, writer)?;

        Self::write_f32_3_to(&self.a, writer)?;
        Self::write_f32_3_to(&self.b, writer)?;
        Self::write_f32_3_to(&self.c, writer)?;

        writer.write_u16::<LE>(self.attr)?;

        Ok(())
    }
}
