use crate::models::{HeroSaveInfo, EquipmentSaveInfo};

pub struct HeroAndEquipment {
    hero: HeroSaveInfo,
    equipment: Vec<EquipmentSaveInfo>,
}