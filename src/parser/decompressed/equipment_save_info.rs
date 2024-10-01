use std::os::linux::raw::stat;

use crate::{MAX_DAMAGE_REDUCTIONS, MAX_LEVELUP_STATS};

use byteorder::{LittleEndian, ReadBytesExt};

use super::LinearColor;
use crate::my_bytes_ext::MyReadBytesExt;

#[derive(Debug)]
pub struct EquipmentSaveInfo {
    is_initialized: bool,

    // maybe handle this in a more rusty way
    damage_reduction_index: [u8; MAX_DAMAGE_REDUCTIONS],
    damage_reduction_percentage: [u8; MAX_DAMAGE_REDUCTIONS],

    stat_modifiers: [i32; MAX_LEVELUP_STATS],
    spawn_stat_modifiers: [i32; MAX_LEVELUP_STATS],

    weapon_damage_bonus: i32,
    weapon_number_of_projectiles_bonus: u8,
    weapon_speed_of_projectiles_bonus: i32,
    weapon_additional_damage_type_index: u8,
    weapon_additional_damage_amount: i32,
    weapon_draw_scale_multiplier: f32,
    weapon_swing_speed_multiplier: f32,

    level: i32,
    stored_mana: i32,

    spawn_quality: f32,
    spawn_randomizer_multiplier: f32,

    weapon_blocking_bonus: u8,
    weapon_alt_damage_bonus: i32,
    weapon_clip_ammo_bonus: i32,
    weapon_reload_speed_bonus: u8,
    weapon_knockback_bonus: u8,
    weapon_charge_speed_bonus: u8,
    weapon_shots_per_second_bonus: u8,

    name_index_base: u8,
    name_index_damage_reduction: u8,
    name_index_quality_descriptor: u8,

    primary_color_set: u8,
    secondary_color_set: u8,

    equipment_id1: i32,
    equipment_id2: i32,

    minimum_sell_value: i32,
    maximum_sell_value: i32,
    max_level: i32,

    dropped_location_x: i32,
    dropped_location_y: i32,
    dropped_location_z: i32,

    can_be_upgraded: bool,
    allow_renaming_at_max_upgrade: bool,

    cant_be_dropped: bool,
    cant_be_sold: bool,
    auto_lock_in_item_box: bool,
    did_one_time_effect: bool,
    is_locked: bool,
    manual_lr: bool, // what is this?

    primary_color_override: LinearColor,
    secondary_color_override: LinearColor,

    user_equipment_name: String,
    user_forger_name: String,
    description: String,
    equipment_template: String,
    equipment_timestamp: String,

    folder_id: i32,
    is_secondary: bool,

    stat_equipment_ids: [i32; 3],
    stat_equipment_tiers: [i32; 3],
    quality_beam_color_override: LinearColor,
}

impl EquipmentSaveInfo {
    pub fn read<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let is_initialized = reader.read_u8()? != 0;

        let mut damage_reduction_index = [0; MAX_DAMAGE_REDUCTIONS];
        for i in 0..MAX_DAMAGE_REDUCTIONS {
            damage_reduction_index[i] = reader.read_u8()?;
        }

        let mut damage_reduction_percentage = [0; MAX_DAMAGE_REDUCTIONS];
        for i in 0..MAX_DAMAGE_REDUCTIONS {
            damage_reduction_percentage[i] = reader.read_u8()?;
        }

        let mut stat_modifiers = [0; MAX_LEVELUP_STATS];
        for i in 0..MAX_LEVELUP_STATS {
            stat_modifiers[i] = reader.read_i32::<LittleEndian>()?;
        }

        let mut spawn_stat_modifiers = [0; MAX_LEVELUP_STATS];
        for i in 0..MAX_LEVELUP_STATS {
            spawn_stat_modifiers[i] = reader.read_i32::<LittleEndian>()?;
        }

        let weapon_damage_bonus = reader.read_i32::<LittleEndian>()?;
        let weapon_number_of_projectiles_bonus = reader.read_u8()?;
        let weapon_speed_of_projectiles_bonus = reader.read_i32::<LittleEndian>()?;
        let weapon_additional_damage_type_index = reader.read_u8()?;
        let weapon_additional_damage_amount = reader.read_i32::<LittleEndian>()?;
        let weapon_draw_scale_multiplier = reader.read_f32::<LittleEndian>()?;
        let weapon_swing_speed_multiplier = reader.read_f32::<LittleEndian>()?;

        let level = reader.read_i32::<LittleEndian>()?;
        let stored_mana = reader.read_i32::<LittleEndian>()?;

        let spawn_quality = reader.read_f32::<LittleEndian>()?;
        let spawn_randomizer_multiplier = reader.read_f32::<LittleEndian>()?;

        let weapon_blocking_bonus = reader.read_u8()?;
        let weapon_alt_damage_bonus = reader.read_i32::<LittleEndian>()?;
        let weapon_clip_ammo_bonus = reader.read_i32::<LittleEndian>()?;
        let weapon_reload_speed_bonus = reader.read_u8()?;
        let weapon_knockback_bonus = reader.read_u8()?;
        let weapon_charge_speed_bonus = reader.read_u8()?;
        let weapon_shots_per_second_bonus = reader.read_u8()?;

        let name_index_base = reader.read_u8()?;
        let name_index_damage_reduction = reader.read_u8()?;
        let name_index_quality_descriptor = reader.read_u8()?;

        let primary_color_set = reader.read_u8()?;
        let secondary_color_set = reader.read_u8()?;

        let equipment_id1 = reader.read_i32::<LittleEndian>()?;
        let equipment_id2 = reader.read_i32::<LittleEndian>()?;

        let minimum_sell_value = reader.read_i32::<LittleEndian>()?;
        let maximum_sell_value = reader.read_i32::<LittleEndian>()?;
        let max_level = reader.read_i32::<LittleEndian>()?;

        let dropped_location_x = reader.read_i32::<LittleEndian>()?;
        let dropped_location_y = reader.read_i32::<LittleEndian>()?;
        let dropped_location_z = reader.read_i32::<LittleEndian>()?;

        let can_be_upgraded = reader.read_u8()? != 0;
        let allow_renaming_at_max_upgrade = reader.read_u8()? != 0;

        let cant_be_dropped = reader.read_u8()? != 0;
        let cant_be_sold = reader.read_u8()? != 0;
        let auto_lock_in_item_box = reader.read_u8()? != 0;
        let did_one_time_effect = reader.read_u8()? != 0;
        let is_locked = reader.read_u8()? != 0;
        let manual_lr = reader.read_u8()? != 0;

        let primary_color_override = LinearColor::read(reader)?;
        let secondary_color_override = LinearColor::read(reader)?;

        let user_equipment_name = reader.read_fstring()?;
        let user_forger_name = reader.read_fstring()?;
        let description = reader.read_fstring()?;
        let equipment_template = reader.read_fstring()?;
        let equipment_timestamp = reader.read_fstring()?;

        let folder_id = reader.read_i32::<LittleEndian>()?;
        let is_secondary = reader.read_u8()? != 0;

        let mut stat_equipment_ids = [0; 3];
        for i in 0..3 {
            stat_equipment_ids[i] = reader.read_i32::<LittleEndian>()?;
        }

        let mut stat_equipment_tiers = [0; 3];
        for i in 0..3 {
            stat_equipment_tiers[i] = reader.read_i32::<LittleEndian>()?;
        }

        let quality_beam_color_override = LinearColor::read(reader)?;

        let equipment_save_info = EquipmentSaveInfo {
            is_initialized,
            damage_reduction_index,
            damage_reduction_percentage,
            stat_modifiers,
            spawn_stat_modifiers,
            weapon_damage_bonus,
            weapon_number_of_projectiles_bonus,
            weapon_speed_of_projectiles_bonus,
            weapon_additional_damage_type_index,
            weapon_additional_damage_amount,
            weapon_draw_scale_multiplier,
            weapon_swing_speed_multiplier,
            level,
            stored_mana,
            spawn_quality,
            spawn_randomizer_multiplier,
            weapon_blocking_bonus,
            weapon_alt_damage_bonus,
            weapon_clip_ammo_bonus,
            weapon_reload_speed_bonus,
            weapon_knockback_bonus,
            weapon_charge_speed_bonus,
            weapon_shots_per_second_bonus,
            name_index_base,
            name_index_damage_reduction,
            name_index_quality_descriptor,
            primary_color_set,
            secondary_color_set,
            equipment_id1,
            equipment_id2,
            minimum_sell_value,
            maximum_sell_value,
            max_level,
            dropped_location_x,
            dropped_location_y,
            dropped_location_z,
            can_be_upgraded,
            allow_renaming_at_max_upgrade,
            cant_be_dropped,
            cant_be_sold,
            auto_lock_in_item_box,
            did_one_time_effect,
            is_locked,
            manual_lr,
            primary_color_override,
            secondary_color_override,
            user_equipment_name,
            user_forger_name,
            description,
            equipment_template,
            equipment_timestamp,
            folder_id,
            is_secondary,
            stat_equipment_ids,
            stat_equipment_tiers,
            quality_beam_color_override,
        };

        Ok(equipment_save_info)
    }
}
