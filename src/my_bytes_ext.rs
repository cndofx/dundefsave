use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

pub trait MyReadBytesExt: ReadBytesExt {
    fn read_fstring(&mut self) -> std::io::Result<String> {
        let length = self.read_u32::<LittleEndian>()?;
        let mut string = String::new();
        for _ in 0..length {
            string.push(self.read_u8()? as char);
        }
        Ok(string)
    }
}

impl<R: Read> MyReadBytesExt for R {}
