use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use super::LinearColor;
use crate::my_bytes_ext::MyReadBytesExt;

#[derive(Debug)]
pub struct HeroSaveInfo {
    is_initialized: bool,

    hero_health_modifier: i32,
    hero_speed_modifier: i32,
    hero_damage_modifier: i32,
    hero_casting_modifier: i32,
    hero_ability1_modifier: i32,
    hero_ability2_modifier: i32,

    hero_defense_health_modifier: i32,
    hero_defense_rate_modifier: i32,
    hero_defense_damage_modifier: i32,
    hero_defense_range_modifier: i32,

    hero_level: i32,
    hero_experience: i32,
    mana_power: i32,

    guid1: i32,
    guid2: i32,
    guid3: i32,
    guid4: i32,

    current_costume_index: i32,
    c1: LinearColor,
    c2: LinearColor,
    c3: LinearColor,

    did_respec: bool,
    gave_exp_bonus: bool,
    allow_rename: bool,

    hero_name: String,
    hero_template: String,

    hotkey_action1: String,
    hotkey_action2: String,
    hotkey_action3: String,
    hotkey_action4: String,
    hotkey_action5: String,
    hotkey_action6: String,
    hotkey_action7: String,
    hotkey_action8: String,
    hotkey_action9: String,
    hotkey_action10: String,

    equipment_count: i32,
}

impl HeroSaveInfo {
    pub fn read<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        let is_initialized = reader.read_u8()? != 0;

        let hero_health_modifier = reader.read_i32::<LittleEndian>()?;
        let hero_speed_modifier = reader.read_i32::<LittleEndian>()?;
        let hero_damage_modifier = reader.read_i32::<LittleEndian>()?;
        let hero_casting_modifier = reader.read_i32::<LittleEndian>()?;
        let hero_ability1_modifier = reader.read_i32::<LittleEndian>()?;
        let hero_ability2_modifier = reader.read_i32::<LittleEndian>()?;

        let hero_defense_health_modifier = reader.read_i32::<LittleEndian>()?;
        let hero_defense_rate_modifier = reader.read_i32::<LittleEndian>()?;
        let hero_defense_damage_modifier = reader.read_i32::<LittleEndian>()?;
        let hero_defense_range_modifier = reader.read_i32::<LittleEndian>()?;

        let hero_level = reader.read_i32::<LittleEndian>()?;
        let hero_experience = reader.read_i32::<LittleEndian>()?;
        let mana_power = reader.read_i32::<LittleEndian>()?;

        let guid1 = reader.read_i32::<LittleEndian>()?;
        let guid2 = reader.read_i32::<LittleEndian>()?;
        let guid3 = reader.read_i32::<LittleEndian>()?;
        let guid4 = reader.read_i32::<LittleEndian>()?;

        let current_costume_index = reader.read_i32::<LittleEndian>()?;
        let c1 = LinearColor::read(reader)?;
        let c2 = LinearColor::read(reader)?;
        let c3 = LinearColor::read(reader)?;

        let did_respec = reader.read_u8()? != 0;
        let gave_exp_bonus = reader.read_u8()? != 0;
        let allow_rename = reader.read_u8()? != 0;

        let hero_name = reader.read_fstring()?;
        let hero_template = reader.read_fstring()?;

        let hotkey_action1 = reader.read_fstring()?;
        let hotkey_action2 = reader.read_fstring()?;
        let hotkey_action3 = reader.read_fstring()?;
        let hotkey_action4 = reader.read_fstring()?;
        let hotkey_action5 = reader.read_fstring()?;
        let hotkey_action6 = reader.read_fstring()?;
        let hotkey_action7 = reader.read_fstring()?;
        let hotkey_action8 = reader.read_fstring()?;
        let hotkey_action9 = reader.read_fstring()?;
        let hotkey_action10 = reader.read_fstring()?;

        let equipment_count = reader.read_i32::<LittleEndian>()?;

        let hero_save_info = Self {
            is_initialized,
            hero_health_modifier,
            hero_speed_modifier,
            hero_damage_modifier,
            hero_casting_modifier,
            hero_ability1_modifier,
            hero_ability2_modifier,
            hero_defense_health_modifier,
            hero_defense_rate_modifier,
            hero_defense_damage_modifier,
            hero_defense_range_modifier,
            hero_level,
            hero_experience,
            mana_power,
            guid1,
            guid2,
            guid3,
            guid4,
            current_costume_index,
            c1,
            c2,
            c3,
            did_respec,
            gave_exp_bonus,
            allow_rename,
            hero_name,
            hero_template,
            hotkey_action1,
            hotkey_action2,
            hotkey_action3,
            hotkey_action4,
            hotkey_action5,
            hotkey_action6,
            hotkey_action7,
            hotkey_action8,
            hotkey_action9,
            hotkey_action10,
            equipment_count,
        };

        dbg!(&hero_save_info);
        Ok(hero_save_info)
    }
}
