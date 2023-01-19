use crate::crate_prelude::*;

#[derive(BingusModuleTrait)]
#[add_bingus_fields]
#[bingus_module(name = "ESP")]
pub struct Esp {

}

impl MakeNewBingusModule for Esp {
    fn new() -> Self {
        Self {
            __enabled_bool_setting: (BingusSetting::BoolSetting(false.into()), "enabled"),
        }
    }
}