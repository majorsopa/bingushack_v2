use crate::crate_prelude::*;


#[derive(BingusModuleTrait)]
#[add_bingus_fields]
#[bingus_module(name = "FakeGhostTotem")]
pub struct FakeGhostTotem {
}

impl MakeNewBingusModule for FakeGhostTotem {
    fn new() -> Self {
        Self {
            __enabled_bool_setting: (BingusSetting::BoolSetting(false.into()), "enabled", None),
            __keybind_setting: (BingusSetting::KeySetting(String::from("").into()), "keybind", None),
            __env: None,
            __mappings_manager: None,
            __prev_enabled: false,
        }
    }
}