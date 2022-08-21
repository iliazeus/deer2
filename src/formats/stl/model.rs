use crate::affine::*;
use crate::geometry::*;

use std::io;
use std::io::{Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt, LE};

use super::*;

#[derive(Debug, Clone)]
pub struct StlModel {
    pub header: String,
    pub triangles: Vec<StlTriangle>,
}

impl Geometry for StlModel {
    type Num = f32;

    fn apply(mut self, xform: &Transform3<f32>) -> Self {
        self.triangles = self.triangles.into_iter().map(|t| t.apply(xform)).collect();
        self
    }
}

impl<'a> TriangleMesh<'a> for StlModel {
    type Triangle = StlTriangle;
    type Triangles = std::slice::Iter<'a, StlTriangle>;

    fn triangles(&'a self) -> Self::Triangles {
        self.triangles.iter()
    }
}

impl StlModel {
    pub fn read_from<R: Read>(reader: &mut R) -> Result<Self, io::Error> {
        let mut header = String::with_capacity(80);
        reader.by_ref().take(80).read_to_string(&mut header)?;

        let triangle_count = reader.read_u32::<LE>()?;

        let mut triangles = Vec::<StlTriangle>::with_capacity(triangle_count as usize);

        for _ in 0..triangle_count {
            let triangle = StlTriangle::read_from(reader)?;
            triangles.push(triangle);
        }

        Ok(Self { header, triangles })
    }

    pub fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), io::Error> {
        let mut header_bytes = self.header.as_bytes().to_vec();
        header_bytes.resize(80, 0);
        writer.write_all(&header_bytes)?;

        // TODO: handle possible overflow?
        let triangle_count = u32::try_from(self.triangles.len()).unwrap();
        writer.write_u32::<LE>(triangle_count)?;

        for triangle in self.triangles.iter() {
            triangle.write_to(writer)?;
        }

        Ok(())
    }
}