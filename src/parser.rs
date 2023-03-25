use deku::prelude::*;

#[derive(Debug, DekuRead)]
#[deku(endian = "little")]
pub struct CompressedSave {
    pub unk00: u32,
    pub version: u32,
    pub decompressed_size: u32,
    pub size: u32,
    pub crc: u32,
    pub magic: u32,
    pub block_size: u32,
    pub compressed_size: u32,
    pub decompressed_size_2: u32,
    pub compressed_size_2: u32,
    pub decompressed_size_3: u32,
    #[deku(count = "compressed_size")]
    pub data: Vec<u8>,
}
