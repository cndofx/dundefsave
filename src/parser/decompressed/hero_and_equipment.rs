use super::{EquipmentSaveInfo, HeroSaveInfo};

#[derive(Debug)]
pub struct HeroAndEquipment {
    hero: HeroSaveInfo,
    equipment: Vec<EquipmentSaveInfo>,
}
