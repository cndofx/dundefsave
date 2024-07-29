use std::io::Cursor;
use std::io::Read;

use byteorder::ReadBytesExt;

use byteorder::LittleEndian;

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
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        let mut rdr = Cursor::new(bytes);

        let unk_version1 = rdr.read_u32::<LittleEndian>().unwrap();
        let unk_version2 = rdr.read_u32::<LittleEndian>().unwrap();
        let decompressed_size = rdr.read_u32::<LittleEndian>().unwrap();
        let size = rdr.read_u32::<LittleEndian>().unwrap();
        let crc = rdr.read_u32::<LittleEndian>().unwrap();
        let magic = rdr.read_u32::<LittleEndian>().unwrap();
        let block_size = rdr.read_u32::<LittleEndian>().unwrap();
        let whole_compressed_size = rdr.read_u32::<LittleEndian>().unwrap();
        let whole_decompressed_size = rdr.read_u32::<LittleEndian>().unwrap();

        let mut block_info = Vec::new();
        let mut total_size = 0;
        while total_size < whole_compressed_size {
            let compressed_size = rdr.read_u32::<LittleEndian>().unwrap();
            let block_size = rdr.read_u32::<LittleEndian>().unwrap();
            block_info.push(BlockInfo {
                compressed_size,
                block_size,
            });
            total_size += compressed_size;
        }

        let mut data = Vec::new();
        for block in block_info.iter() {
            let mut block_data = vec![0; block.compressed_size as usize];
            rdr.read_exact(&mut block_data).unwrap();
            data.push(block_data);
        }

        Self {
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
        }
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
