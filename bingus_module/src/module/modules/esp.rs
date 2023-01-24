use crate::crate_prelude::*;


fn tick(esp: &mut Esp, env: JNIEnv, mappings_manager: &MappingsManager) {

}

fn render(_esp: &mut Esp) {

}

#[derive(BingusModuleTrait)]
#[add_bingus_fields]
#[bingus_module(name = "ESP (doesn't work)", tick_method = "tick(self, _env, _mappings_manager)", render_method = "render(self)")]
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