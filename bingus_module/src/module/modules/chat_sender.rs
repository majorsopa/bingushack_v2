use crate::{crate_prelude::*, module::bingus_module_trait::MakeNewBingusModule};


fn tick(env: JNIEnv, mappings_manager: Rc<MappingsManager>) {
    println!("tick called");
}


#[derive(BingusModuleTrait)]
#[add_bingus_fields]
#[bingus_module(name = "ChatSender", tick_method = "tick(_env, _mappings_manager)", settings_list_field_names = "[bool_setting]")]
pub struct ChatSender {
    bool_setting: (BingusSetting, &'static str),
}

impl MakeNewBingusModule for ChatSender {
    fn new() -> Self {
        let new_self = Self {
            bool_setting: (BingusSetting::BoolSetting(false.into()), "testing name"),
            __enabled_bool_setting: (BingusSetting::BoolSetting(false.into()), "enabled"),
        };

        new_self
    }
}
