use std::io::Read;

use super::{EquipmentSaveInfo, HeroSaveInfo};
use crate::my_bytes_ext::MyReadBytesExt;

#[derive(Debug)]
pub struct HeroAndEquipment {
    hero: HeroSaveInfo,
    equipment: Vec<EquipmentSaveInfo>,
}

impl HeroAndEquipment {
    pub fn read<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        let hero = HeroSaveInfo::read(reader)?;
        let equipment = reader.read_tarray(|reader| EquipmentSaveInfo::read(reader))?;
        dbg!(&equipment);

        Ok(HeroAndEquipment { hero, equipment })
    }
}
