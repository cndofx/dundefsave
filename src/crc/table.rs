use std::ops::Index;

const POLYNOMIAL: u32 = 0x04C11DB7;

#[derive(Debug)]
pub struct CRCLookupTable {
    data: [u32; 256],
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

        CRCLookupTable { data }
    }
}

impl Index<usize> for CRCLookupTable {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}