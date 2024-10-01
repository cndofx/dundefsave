use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
pub struct LinearColor {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl LinearColor {
    pub fn read<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        let r = reader.read_f32::<LittleEndian>()?;
        let g = reader.read_f32::<LittleEndian>()?;
        let b = reader.read_f32::<LittleEndian>()?;
        let a = reader.read_f32::<LittleEndian>()?;
        Ok(LinearColor { r, g, b, a })
    }
}
