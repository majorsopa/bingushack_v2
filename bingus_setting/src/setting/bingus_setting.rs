use crate::crate_prelude::*;
pub use settings::*;

#[derive(Clone)]
#[enum_dispatch(BingusSettingTrait)]
pub enum BingusSetting {
    BoolSetting(BoolSetting),
    IntSetting(IntSetting),
    KeySetting(KeySetting),
    FloatSetting(FloatSetting),
    RangeSetting(RangeSetting),
}

impl BingusSetting {
    pub fn get_bool(&mut self) -> &mut bool {
        match self {
            BingusSetting::BoolSetting(setting) => setting.get_value(),
            _ => panic!("Not a bool setting"),
        }
    }

    pub fn get_int(&mut self) -> &mut u32 {
        match self {
            BingusSetting::IntSetting(setting) => setting.get_value(),
            _ => panic!("Not a int setting"),
        }
    }

    pub fn get_key(&mut self) -> &mut String {
        match self {
            BingusSetting::KeySetting(setting) => setting.get_value(),
            _ => panic!("Not a key setting"),
        }
    }

    pub fn get_float(&mut self) -> &mut f32 {
        match self {
            BingusSetting::FloatSetting(setting) => setting.get_value(),
            _ => panic!("Not a float setting"),
        }
    }

    pub fn get_range(&mut self) -> &mut [f32; 2] {
        match self {
            BingusSetting::RangeSetting(setting) => setting.get_value(),
            _ => panic!("Not a range setting"),
        }
    }
}



mod settings {
    use crate::crate_prelude::*;


    #[derive(BingusSettingTrait, Clone)]
    #[bingus_setting(setting_type = "bool")]
    pub struct BoolSetting(Arc<bool>);

    impl From<bool> for BoolSetting {
        fn from(value: bool) -> Self {
            Self(Arc::new(value))
        }
    }

    #[derive(BingusSettingTrait, Clone)]
    #[bingus_setting(setting_type = "u32")]
    pub struct IntSetting(Arc<u32>);

    impl From<u32> for IntSetting {
        fn from(value: u32) -> Self {
            Self(Arc::new(value))
        }
    }

    #[derive(BingusSettingTrait, Clone)]
    #[bingus_setting(setting_type = "f32")]
    pub struct FloatSetting(Arc<f32>);

    impl From<f32> for FloatSetting {
        fn from(value: f32) -> Self {
            Self(Arc::new(value))
        }
    }

    #[derive(BingusSettingTrait, Clone)]
    #[bingus_setting(setting_type = "[f32; 2]")]
    pub struct RangeSetting(Arc<[f32; 2]>);

    impl From<[f32; 2]> for RangeSetting {
        fn from(value: [f32; 2]) -> Self {
            Self(Arc::new(value))
        }
    }

    #[derive(BingusSettingTrait, Clone)]
    #[bingus_setting(setting_type = "String")]
    pub struct KeySetting(Arc<String>);

    impl From<String> for KeySetting {
        fn from(value: String) -> Self {
            Self(Arc::new(value))
        }
    }
}