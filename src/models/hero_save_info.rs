use crate::models::LinearColor;

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
}