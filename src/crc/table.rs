const POLYNOMIAL: u32 = 0x04C11DB7;

#[derive(Debug)]
pub struct CRCLookupTable {
    data: [u8; 1024],
}

impl CRCLookupTable {
    pub fn new() -> Self {
        let mut data = [0; 256];

        for dividend in 0..256 {
            let mut remainder = dividend << 0x18;

            for _ in 0..8 {
                if remainder & (1 << 31) == 0 {
                    remainder = remainder << 1;
                } else {
                    remainder = (remainder << 1) ^ POLYNOMIAL;
                }
            }

            data[dividend as usize] = remainder;
        }

        // convert u32 values to u8
        let mut new_data = [0; 1024];
        for i in 0..256 {
            new_data[4*i..][..4].copy_from_slice(&data[i].to_le_bytes());
        }

        CRCLookupTable { data: new_data }
    }
}