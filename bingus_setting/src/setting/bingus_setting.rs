use crate::crate_prelude::*;
pub use settings::*;

#[derive(Clone, Copy)]
#[enum_dispatch(BingusSettingTrait)]
pub enum BingusSetting {
    BoolSetting,
}


mod settings {
    use crate::crate_prelude::*;


    #[derive(BingusSettingTrait, Clone, Copy)]
    #[bingus_setting(setting_type = "bool")]
    pub struct BoolSetting(bool);

    #[derive(BingusSettingTrait, Clone, Copy)]
    #[bingus_setting(setting_type = "u32")]
    pub struct IntSetting(u32);

    #[derive(BingusSettingTrait, Clone, Copy)]
    #[bingus_setting(setting_type = "f32")]
    pub struct FloatSetting(f32);

    #[derive(BingusSettingTrait, Clone, Copy)]
    #[bingus_setting(setting_type = "[f32; 2]")]
    pub struct RangeSetting([f32; 2]);
}