use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

pub trait MyReadBytesExt: ReadBytesExt {
    fn read_fstring(&mut self) -> std::io::Result<String> {
        let length = self.read_u32::<LittleEndian>()?;
        let mut string = String::with_capacity(length as usize);
        for _ in 0..length {
            string.push(self.read_u8()? as char);
        }
        if string.len() > 0 {
            assert!(string.ends_with('\0'));
            string.truncate(string.len() - 1);
        }
        dbg!(&string);
        Ok(string)
    }

    fn read_tarray<T>(
        &mut self,
        read: impl Fn(&mut Self) -> std::io::Result<T>,
    ) -> std::io::Result<Vec<T>> {
        let length = self.read_u32::<LittleEndian>()?;
        let mut array = Vec::with_capacity(length as usize);
        for _ in 0..length {
            array.push(read(self)?);
        }
        Ok(array)
    }
}

impl<R: Read> MyReadBytesExt for R {}
