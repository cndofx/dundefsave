use crate::{MAX_DAMAGE_REDUCTIONS, MAX_LEVELUP_STATS};

use crate::models::LinearColor;

pub struct EquipmentSaveInfo {
    is_initialized: bool,

    // maybe handle this in a more rusty way
    damage_reduction_index: [u8; MAX_DAMAGE_REDUCTIONS],
    damage_reduction_percentage: [u8; MAX_DAMAGE_REDUCTIONS],
    
    stat_modifiers: [i32; MAX_LEVELUP_STATS],

    weapon_damage_bonus: i32,
    weapon_number_of_projectiles_bonus: u8,
    weapon_speed_of_projectiles_bonus: i32,
    weapon_additional_damage_type_index: u8,
    weapon_additional_damage_amount: i32,
    weapon_draw_scale_multiplier: f32,
    weapon_swing_speed_multiplier: f32,

    level: i32,
    stored_mana: i32,
    
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

    folder_id: i32,
    is_secondary: bool,
}