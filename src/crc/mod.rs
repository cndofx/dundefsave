use self::table::CRCLookupTable;

pub mod table;

pub fn crc32(data: &[u8], lookup_table: &CRCLookupTable) -> u32 {
    let mut sum: u32 = 0xFFFFFFFF;
    for byte in data.iter() {
        let table_idx = (*byte as u32) ^ (sum >> 24);
        sum = sum << 8;
        sum = sum ^ lookup_table[table_idx as usize];
    }
    !sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc32() {
        let data: [u8; 16] = [
            0xFB, 0x97, 0xA3, 0x87, 0x58, 0xC1, 0x05, 0x45, 0x84, 0xCD, 0x5A, 0xAC, 0x4E, 0x16,
            0x0E, 0xEF,
        ];

        let table = CRCLookupTable::new();
        let checksum = crc32(&data, &table);

        assert_eq!(checksum, 0x6953FF9D);
    }
}
