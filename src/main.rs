use std::io::Read;

use deku::DekuContainerRead;
use dundefsave::{crc, parser::CompressedSave};

fn main() {
    let table = crc::table::CRCLookupTable::new();
    println!("{:08X?}", table);

    let data = std::fs::read("savedata/DunDefHeroes.dun").unwrap();
    let (_, save) = CompressedSave::from_bytes((&data, 0)).unwrap();
    println!("{:X?}", save);

    let mut decompressed = Vec::with_capacity(save.decompressed_size as usize);
    compress::zlib::Decoder::new(&*save.data)
        .read_to_end(&mut decompressed)
        .unwrap();

    println!("{:X?}", decompressed);
}
