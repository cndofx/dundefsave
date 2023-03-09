use crate::MAX_TUTORIAL_SETS;

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

pub struct OptionsFixedSize {
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

    // for online players joining
    minimum_level: i32,

    // cached gamespy login info
    saved_login_info: bool,
    custom_game_meta_flags: Vec<u8>,

    costume_unlocks: Vec<i32>,
    hero_unlocks: Vec<i32>,
}

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
