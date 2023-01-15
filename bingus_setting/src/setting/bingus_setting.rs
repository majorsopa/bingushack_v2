use crate::crate_prelude::*;
pub use settings::*;
pub use settings_types::*;

#[derive(Clone, Copy)]
#[enum_dispatch(BingusSettingTrait)]
pub enum BingusSetting {
    BoolSetting,
}

impl BingusSetting {
    pub fn get_value(&self) -> SettingsType {
        match self {
            BingusSetting::BoolSetting(setting) => setting.get_value().into(),
        }
    }

    pub fn get_value_mut(&mut self) -> &mut SettingsType {
        match self {
            BingusSetting::BoolSetting(setting) => setting.get_value_mut().into(),
        }
    }
}



// todo make all a macro
mod settings_types {
    pub enum SettingsType {
        Bool(bool),
        Int(u32),
        Float(f32),
        Range([f32; 2]),
    }

    impl From<bool> for SettingsType {
        fn from(value: bool) -> Self {
            SettingsType::Bool(value)
        }
    }

    impl From<&mut bool> for &mut SettingsType {
        fn from(value: &mut bool) -> Self {
            unsafe { std::mem::transmute(value) }
        }
    }

    impl Into<bool> for SettingsType {
        fn into(self) -> bool {
            match self {
                SettingsType::Bool(value) => value,
                _ => panic!("tried to convert non bool setting to bool"),
            }
        }
    }

    impl<'a> Into<&'a mut bool> for &mut SettingsType {
        fn into(self) -> &'a mut bool {
            match self {
                SettingsType::Bool(value) => unsafe { std::mem::transmute(value) },
                _ => panic!("tried to convert non bool setting to bool"),
            }
        }
    }

    impl From<u32> for SettingsType {
        fn from(value: u32) -> Self {
            SettingsType::Int(value)
        }
    }

    impl From<&mut u32> for &mut SettingsType {
        fn from(value: &mut u32) -> Self {
            unsafe { std::mem::transmute(value) }
        }
    }

    impl Into<u32> for SettingsType {
        fn into(self) -> u32 {
            match self {
                SettingsType::Int(value) => value,
                _ => panic!("tried to convert non int setting to int"),
            }
        }
    }

    impl<'a> Into<&'a mut u32> for &mut SettingsType {
        fn into(self) -> &'a mut u32 {
            match self {
                SettingsType::Int(value) => unsafe { std::mem::transmute(value) },
                _ => panic!("tried to convert non int setting to int"),
            }
        }
    }

    impl From<f32> for SettingsType {
        fn from(value: f32) -> Self {
            SettingsType::Float(value)
        }
    }

    impl From<&mut f32> for &mut SettingsType {
        fn from(value: &mut f32) -> Self {
            unsafe { std::mem::transmute(value) }
        }
    }

    impl Into<f32> for SettingsType {
        fn into(self) -> f32 {
            match self {
                SettingsType::Float(value) => value,
                _ => panic!("tried to convert non float setting to float"),
            }
        }
    }

    impl<'a> Into<&'a mut f32> for &mut SettingsType {
        fn into(self) -> &'a mut f32 {
            match self {
                SettingsType::Float(value) => unsafe { std::mem::transmute(value) },
                _ => panic!("tried to convert non float setting to float"),
            }
        }
    }

    impl From<[f32; 2]> for SettingsType {
        fn from(value: [f32; 2]) -> Self {
            SettingsType::Range(value)
        }
    }

    impl From<&mut [f32; 2]> for &mut SettingsType {
        fn from(value: &mut [f32; 2]) -> Self {
            unsafe { std::mem::transmute(value) }
        }
    }

    impl Into<[f32; 2]> for SettingsType {
        fn into(self) -> [f32; 2] {
            match self {
                SettingsType::Range(value) => value,
                _ => panic!("tried to convert non range setting to range"),
            }
        }
    }

    impl<'a> Into<&'a mut [f32; 2]> for &mut SettingsType {
        fn into(self) -> &'a mut [f32; 2] {
            match self {
                SettingsType::Range(value) => unsafe { std::mem::transmute(value) },
                _ => panic!("tried to convert non range setting to range"),
            }
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