use crate::{crate_prelude::*, module::bingus_module_trait::MakeNewBingusModule};


fn tick() {
    println!("tick called");
}


#[derive(BingusModuleTrait)]
#[add_bingus_fields]
#[bingus_module(name = "ChatSender", tick_method = "tick()", settings_list_field_names = "[bool_setting]")]
pub struct ChatSender {
    bool_setting: BingusSetting,
}

impl MakeNewBingusModule for ChatSender {
    fn new() -> Self {
        let new_self = Self {
            bool_setting: BingusSetting::BoolSetting(false.into()),
            __enabled_bool_setting: BingusSetting::BoolSetting(false.into()),
        };

        new_self
    }
}
