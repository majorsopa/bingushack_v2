use crate::crate_prelude::*;
pub use settings::*;

#[enum_dispatch(BingusSettingTrait)]
pub enum BingusSetting {
    BoolSetting,
}


mod settings {
    use crate::crate_prelude::*;


    #[derive(BingusSettingTrait)]
    #[bingus_setting(setting_type = "bool")]
    pub struct BoolSetting(bool);
}