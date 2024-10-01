use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::my_bytes_ext::MyReadBytesExt;
use crate::MAX_TUTORIAL_SETS;

#[derive(Debug)]
pub struct OptionsInfo {
    fixed_size_options: OptionsFixedSize,
    resolution: String,

    last_level_tag: String,

    // cached gamespy login info
    gamespy_username: String,
    gamespy_password: String,

    search_filters: SearchFilterSettings,
    installed_dlc_equipments: Vec<i32>,
}

#[derive(Debug)]
pub struct OptionsFixedSize {
    unk: u64,

    auto_show_levelup: bool,
    allow_friendly_fire: bool,
    use_gamepad: bool,
    auto_adjust_camera_for_phase: bool,

    show_tutorials: bool,
    shown_tutorials: [i32; MAX_TUTORIAL_SETS],

    volume_sfx: f32,
    volume_music: f32,

    voice_play_volume: f32,
    voice_capture_volume: f32,
    push_to_talk: bool,
    incoming_voice: bool,
    outgoing_voice: bool,

    gamma: f32,
    saturation: f32,
    ui_scale: f32,

    post_processing: bool,

    show_floating_damage_numbers: bool,
    right_strick_turns_camera_scheme: bool,
    invert_camera_pitch: bool,
    swap_triggers_and_buttons: bool,
    fullscreen: bool,
    splitscreen_config: u8,
    current_difficulty: u8,
    lobby_item_lock: bool,
    default_chase_camera: bool,
    default_camera_target_distance: f32,
    default_placing_tower_camera_distance: f32,
    mouse_camera_rotation_speed: f32,

    // new stuff?
    item_quality_filter: u32,
    hide_accessory: bool,
    enable_outline_effect: bool,
    graphics_quality: u8,
    frame_rate_limit: f32,
    inventory_sorting_filter: u8,

    // for online players joining
    minimum_level: i32,

    // cached gamespy login info
    saved_login_info: bool,

    custom_game_meta_flags: Vec<u8>,
    costume_unlocks: Vec<i32>,
    hero_unlocks: Vec<i32>,
}

#[derive(Debug)]
pub struct SearchFilterSettings {
    level_indices_to_filter: Vec<i32>,
    difficulties_to_filter: Vec<i32>,
    filter_challenge_missions: u8,
    filter_campaign_missions: u8,
    filter_pure_strategy: u8,
    filter_infinite_build: u8,
    filter_infinite_waves: u8,

    filter_host_class: u8,
    filter_host_level: u8,
    filter_host_level_start: u8,
    filter_host_level_end: u8,
}

impl OptionsInfo {
    pub fn read<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        let fixed_size_options = OptionsFixedSize::read(reader)?;
        let resolution = reader.read_fstring()?;
        let last_level_tag = reader.read_fstring()?;
        let gamespy_username = reader.read_fstring()?;
        let gamespy_password = reader.read_fstring()?;
        let search_filters = SearchFilterSettings::read(reader)?;

        let mut installed_dlc_equipments = Vec::new();
        let installed_dlc_equipments_length = reader.read_u32::<LittleEndian>()?;
        for _ in 0..installed_dlc_equipments_length {
            installed_dlc_equipments.push(reader.read_i32::<LittleEndian>()?);
        }

        let options_info = OptionsInfo {
            fixed_size_options,
            resolution,
            last_level_tag,
            gamespy_username,
            gamespy_password,
            search_filters,
            installed_dlc_equipments,
        };

        dbg!(&options_info);
        Ok(options_info)
    }
}

impl OptionsFixedSize {
    pub fn read<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        let unk = reader.read_u64::<LittleEndian>()?;

        let auto_show_levelup = reader.read_u8()? != 0;
        let allow_friendly_fire = reader.read_u8()? != 0;
        let use_gamepad = reader.read_u8()? != 0;
        let auto_adjust_camera_for_phase = reader.read_u8()? != 0;

        let show_tutorials = reader.read_u8()? != 0;
        let mut shown_tutorials = [0; MAX_TUTORIAL_SETS];
        for tut in shown_tutorials.iter_mut() {
            *tut = reader.read_i32::<LittleEndian>()?;
        }

        let volume_sfx = reader.read_f32::<LittleEndian>()?;
        let volume_music = reader.read_f32::<LittleEndian>()?;
        let voice_play_volume = reader.read_f32::<LittleEndian>()?;
        let voice_capture_volume = reader.read_f32::<LittleEndian>()?;

        let push_to_talk = reader.read_u8()? != 0;
        let incoming_voice = reader.read_u8()? != 0;
        let outgoing_voice = reader.read_u8()? != 0;

        let gamma = reader.read_f32::<LittleEndian>()?;
        let saturation = reader.read_f32::<LittleEndian>()?;
        let ui_scale = reader.read_f32::<LittleEndian>()?;

        let post_processing = reader.read_u8()? != 0;

        let show_floating_damage_numbers = reader.read_u8()? != 0;
        let right_strick_turns_camera_scheme = reader.read_u8()? != 0;
        let invert_camera_pitch = reader.read_u8()? != 0;
        let swap_triggers_and_buttons = reader.read_u8()? != 0;
        let fullscreen = reader.read_u8()? != 0;
        let splitscreen_config = reader.read_u8()?;
        let current_difficulty = reader.read_u8()?;
        let lobby_item_lock = reader.read_u8()? != 0;
        let default_chase_camera = reader.read_u8()? != 0;
        let default_camera_target_distance = reader.read_f32::<LittleEndian>()?;
        let default_placing_tower_camera_distance = reader.read_f32::<LittleEndian>()?;
        let mouse_camera_rotation_speed = reader.read_f32::<LittleEndian>()?;

        let item_quality_filter = reader.read_u32::<LittleEndian>()?;
        let hide_accessory = reader.read_u8()? != 0;
        let enable_outline_effect = reader.read_u8()? != 0;
        let graphics_quality = reader.read_u8()?;
        let frame_rate_limit = reader.read_f32::<LittleEndian>()?;
        let inventory_sorting_filter = reader.read_u8()?;

        let minimum_level = reader.read_i32::<LittleEndian>()?;

        let saved_login_info = reader.read_u8()? != 0;

        let mut custom_game_meta_flags = Vec::new();
        let custom_game_meta_flags_length = reader.read_u32::<LittleEndian>()?;
        for _ in 0..custom_game_meta_flags_length {
            custom_game_meta_flags.push(reader.read_u8()?);
        }

        let mut costume_unlocks = Vec::new();
        let costume_unlocks_length = reader.read_u32::<LittleEndian>()?;
        for _ in 0..costume_unlocks_length {
            costume_unlocks.push(reader.read_i32::<LittleEndian>()?);
        }

        let mut hero_unlocks = Vec::new();
        let hero_unlocks_length = reader.read_u32::<LittleEndian>()?;
        for _ in 0..hero_unlocks_length {
            hero_unlocks.push(reader.read_i32::<LittleEndian>()?);
        }

        let options_fixed_size = OptionsFixedSize {
            unk,
            auto_show_levelup,
            allow_friendly_fire,
            use_gamepad,
            auto_adjust_camera_for_phase,
            show_tutorials,
            shown_tutorials,
            volume_sfx,
            volume_music,
            voice_play_volume,
            voice_capture_volume,
            push_to_talk,
            incoming_voice,
            outgoing_voice,
            gamma,
            saturation,
            ui_scale,
            post_processing,
            show_floating_damage_numbers,
            right_strick_turns_camera_scheme,
            invert_camera_pitch,
            swap_triggers_and_buttons,
            fullscreen,
            splitscreen_config,
            current_difficulty,
            lobby_item_lock,
            default_chase_camera,
            default_camera_target_distance,
            default_placing_tower_camera_distance,
            mouse_camera_rotation_speed,
            item_quality_filter,
            hide_accessory,
            enable_outline_effect,
            graphics_quality,
            frame_rate_limit,
            inventory_sorting_filter,
            minimum_level,
            saved_login_info,
            custom_game_meta_flags,
            costume_unlocks,
            hero_unlocks,
        };

        dbg!(&options_fixed_size);
        Ok(options_fixed_size)
    }
}

impl SearchFilterSettings {
    pub fn read<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        let level_indices_to_filter_length = reader.read_u32::<LittleEndian>()?;
        let mut level_indices_to_filter = Vec::new();
        for _ in 0..level_indices_to_filter_length {
            level_indices_to_filter.push(reader.read_i32::<LittleEndian>()?);
        }

        let difficulties_to_filter_length = reader.read_u32::<LittleEndian>()?;
        let mut difficulties_to_filter = Vec::new();
        for _ in 0..difficulties_to_filter_length {
            difficulties_to_filter.push(reader.read_i32::<LittleEndian>()?);
        }

        let filter_challenge_missions = reader.read_u8()?;
        let filter_campaign_missions = reader.read_u8()?;
        let filter_pure_strategy = reader.read_u8()?;
        let filter_infinite_build = reader.read_u8()?;
        let filter_infinite_waves = reader.read_u8()?;
        let filter_host_class = reader.read_u8()?;
        let filter_host_level = reader.read_u8()?;
        let filter_host_level_start = reader.read_u8()?;
        let filter_host_level_end = reader.read_u8()?;

        Ok(SearchFilterSettings {
            level_indices_to_filter,
            difficulties_to_filter,
            filter_challenge_missions,
            filter_campaign_missions,
            filter_pure_strategy,
            filter_infinite_build,
            filter_infinite_waves,
            filter_host_class,
            filter_host_level,
            filter_host_level_start,
            filter_host_level_end,
        })
    }
}

// #[derive(Debug)]
// pub struct SearchFilterSettings {
//     level_indices_to_filter: Vec<i32>,
//     difficulties_to_filter: Vec<i32>,
//     filter_challenge_missions: u8,
//     filter_campaign_missions: u8,
//     filter_pure_strategy: u8,
//     filter_infinite_build: u8,
//     filter_infinite_waves: u8,

//     filter_host_class: u8,
//     filter_host_level: u8,
//     filter_host_level_start: u8,
//     filter_host_level_end: u8,
// }
