use std::io::Cursor;
use std::io::Read;

use byteorder::ReadBytesExt;

use byteorder::LittleEndian;

pub mod decompressed;

pub struct CompressedSave {
    pub unk_version1: u32,
    pub unk_version2: u32,
    pub decompressed_size: u32,
    pub size: u32,
    pub crc: u32,
    pub magic: u32,
    pub block_size: u32,
    pub whole_compressed_size: u32,
    pub whole_decompressed_size: u32,
    pub block_info: Vec<BlockInfo>,
    pub data: Vec<Vec<u8>>,
}

pub struct BlockInfo {
    pub compressed_size: u32,
    pub block_size: u32,
}

impl CompressedSave {
    // pub fn from_bytes(bytes: Vec<u8>) -> Self {
    // let mut rdr = Cursor::new(bytes);
    pub fn read<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        let unk_version1 = reader.read_u32::<LittleEndian>()?;
        let unk_version2 = reader.read_u32::<LittleEndian>()?;
        let decompressed_size = reader.read_u32::<LittleEndian>()?;
        let size = reader.read_u32::<LittleEndian>()?;
        let crc = reader.read_u32::<LittleEndian>()?;
        let magic = reader.read_u32::<LittleEndian>()?;
        let block_size = reader.read_u32::<LittleEndian>()?;
        let whole_compressed_size = reader.read_u32::<LittleEndian>()?;
        let whole_decompressed_size = reader.read_u32::<LittleEndian>()?;

        let mut block_info = Vec::new();
        let mut total_size = 0;
        while total_size < whole_compressed_size {
            let compressed_size = reader.read_u32::<LittleEndian>()?;
            let block_size = reader.read_u32::<LittleEndian>()?;
            block_info.push(BlockInfo {
                compressed_size,
                block_size,
            });
            total_size += compressed_size;
        }

        let mut data = Vec::new();
        for block in block_info.iter() {
            let mut block_data = vec![0; block.compressed_size as usize];
            reader.read_exact(&mut block_data)?;
            data.push(block_data);
        }

        let save = Self {
            unk_version1,
            unk_version2,
            decompressed_size,
            size,
            crc,
            magic,
            block_size,
            whole_compressed_size,
            whole_decompressed_size,
            block_info,
            data,
        };

        Ok(save)
    }
}

// uint32 unk_version1;
// uint32 unk_version2;
// uint32 decompressedSize;
// uint32 size;
// uint32 crc;
// uint32 magic;
// uint32 blockSize;
// uint32 wholeCompressedSize;
// uint32 wholeDecompressedSize;

// typedef struct {
//     uint32 compressedSize;
//     uint32 blockSize;
// } BlockInfo;

// // read blocks until the sum of compressedSize == wholeCompressedSize
// BlockInfo block1;
// BlockInfo block2;
// BlockInfo block3;
// BlockInfo block4;
// BlockInfo block5;
// BlockInfo block6;

// use deku::prelude::*;

// #[derive(Debug, DekuRead)]
// #[deku(endian = "little")]
// pub struct CompressedSave {
//     pub unk00: u32,
//     pub version: u32,
//     pub decompressed_size: u32,
//     pub size: u32,
//     pub crc: u32,
//     pub magic: u32,
//     pub block_size: u32,
//     pub compressed_size: u32,
//     pub decompressed_size_2: u32,
//     pub compressed_size_2: u32,
//     pub decompressed_size_3: u32,
//     #[deku(count = "compressed_size")]
//     pub data: Vec<u8>,
// }
