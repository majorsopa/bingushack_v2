use crate::crate_prelude::*;
pub use settings::*;

#[derive(Clone, Copy)]
#[enum_dispatch(BingusSettingTrait)]
pub enum BingusSetting {
    BoolSetting,
    IntSetting,
    FloatSetting,
    RangeSetting,
}

impl BingusSetting {
    pub fn get_bool(&self) -> bool {
        match self {
            BingusSetting::BoolSetting(setting) => setting.get_value(),
            _ => panic!("Not a bool setting"),
        }
    }

    pub fn get_bool_mut(&mut self) -> &mut bool {
        match self {
            BingusSetting::BoolSetting(setting) => setting.get_value_mut(),
            _ => panic!("Not a bool setting"),
        }
    }

    pub fn get_int(&self) -> u32 {
        match self {
            BingusSetting::IntSetting(setting) => setting.get_value(),
            _ => panic!("Not a int setting"),
        }
    }

    pub fn get_int_mut(&mut self) -> &mut u32 {
        match self {
            BingusSetting::IntSetting(setting) => setting.get_value_mut(),
            _ => panic!("Not a int setting"),
        }
    }

    pub fn get_float(&self) -> f32 {
        match self {
            BingusSetting::FloatSetting(setting) => setting.get_value(),
            _ => panic!("Not a float setting"),
        }
    }

    pub fn get_float_mut(&mut self) -> &mut f32 {
        match self {
            BingusSetting::FloatSetting(setting) => setting.get_value_mut(),
            _ => panic!("Not a float setting"),
        }
    }

    pub fn get_range(&self) -> [f32; 2] {
        match self {
            BingusSetting::RangeSetting(setting) => setting.get_value(),
            _ => panic!("Not a range setting"),
        }
    }

    pub fn get_range_mut(&mut self) -> &mut [f32; 2] {
        match self {
            BingusSetting::RangeSetting(setting) => setting.get_value_mut(),
            _ => panic!("Not a range setting"),
        }
    }
}



mod settings {
    use crate::crate_prelude::*;


    #[derive(BingusSettingTrait, Clone, Copy)]
    #[bingus_setting(setting_type = "bool")]
    pub struct BoolSetting(bool);

    impl From<bool> for BoolSetting {
        fn from(value: bool) -> Self {
            Self(value)
        }
    }

    #[derive(BingusSettingTrait, Clone, Copy)]
    #[bingus_setting(setting_type = "u32")]
    pub struct IntSetting(u32);

    impl From<u32> for IntSetting {
        fn from(value: u32) -> Self {
            Self(value)
        }
    }

    #[derive(BingusSettingTrait, Clone, Copy)]
    #[bingus_setting(setting_type = "f32")]
    pub struct FloatSetting(f32);

    impl From<f32> for FloatSetting {
        fn from(value: f32) -> Self {
            Self(value)
        }
    }

    #[derive(BingusSettingTrait, Clone, Copy)]
    #[bingus_setting(setting_type = "[f32; 2]")]
    pub struct RangeSetting([f32; 2]);

    impl From<[f32; 2]> for RangeSetting {
        fn from(value: [f32; 2]) -> Self {
            Self(value)
        }
    }
}