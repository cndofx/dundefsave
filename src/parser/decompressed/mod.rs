pub mod equipment_save_info;
pub mod hero_and_equipment;
pub mod hero_save_info;
pub mod linear_color;
pub mod options_info;

use std::io::Read;

use byteorder::ReadBytesExt;

use byteorder::LittleEndian;
use equipment_save_info::EquipmentSaveInfo;
use hero_and_equipment::HeroAndEquipment;
use hero_save_info::HeroSaveInfo;
use linear_color::LinearColor;
use options_info::OptionsInfo;

#[derive(Debug)]
pub struct GameStorage {
    game_options: OptionsInfo,
    heroes: Vec<HeroAndEquipment>,
}

impl GameStorage {
    pub fn read<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        let game_options = OptionsInfo::read(reader)?;

        todo!();

        // let heroes_length = reader.read_u32::<LittleEndian>()?;
        // let mut heroes = Vec::new();
        // for _ in 0..heroes_length {
        //     heroes.push(HeroAndEquipment::read(reader)?);
        // }

        // let game_storage = GameStorage {
        //     game_options,
        //     heroes,
        // };
        // Ok(game_storage)
    }
}
