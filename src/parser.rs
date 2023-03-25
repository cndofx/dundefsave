use deku::prelude::*;

#[derive(Debug, DekuRead)]
#[deku(endian = "little")]
pub struct CompressedSave {
    unk00: u32,
    version: u32,
    decompressed_size: u32,
    size: u32,
    crc: u32,
    magic: u32,
    block_size: u32,
    compressed_size: u32,
    decompressed_size_2: u32,
    compressed_size_2: u32,
    decompressed_size_3: u32,
    #[deku(count = "compressed_size")]
    data: Vec<u8>,
}
