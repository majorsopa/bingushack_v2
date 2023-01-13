use crate::crate_prelude::*;


fn tick() {
    println!("tick called");
}


#[derive(BingusModuleTrait)]
#[add_bingus_fields]
#[bingus_module(name = "ChatSender", tick_method = "tick()", settings_list_field_names = "[bool_setting]")]
pub struct ChatSender {
    bool_setting: BingusSetting,
}
