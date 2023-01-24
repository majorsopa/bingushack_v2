use crate::crate_prelude::*;


fn render(_esp: &mut Esp, _env: JNIEnv, _mappings_manager: &MappingsManager) {

}

#[derive(BingusModuleTrait)]
#[add_bingus_fields]
#[bingus_module(name = "ESP (doesn't work)", render_method = "render(self, _env, _mappings_manager)")]
pub struct Esp {
    
}

impl MakeNewBingusModule for Esp {
    fn new() -> Self {
        Self {
            __enabled_bool_setting: (BingusSetting::BoolSetting(false.into()), "enabled", None),
            __keybind_setting: (BingusSetting::KeySetting(String::from("").into()), "keybind", None),
            __env: None,
            __mappings_manager: None,
        }
    }
}