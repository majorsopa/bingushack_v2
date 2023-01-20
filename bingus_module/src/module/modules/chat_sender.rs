use crate::crate_prelude::*;


fn tick(env: JNIEnv, mappings_manager: &MappingsManager) {
    let minecraft_client = get_minecraft_client(env, mappings_manager);

    let player = match get_player_checked(env, mappings_manager, minecraft_client) {
        Some(player) => player,
        None => return,
    };

    let bingus_text = make_minecraft_text_object(env, mappings_manager, "bingus");

    call_method_or_get_field!(env, player, "displayClientMessage", false, &[
        JValue::from(bingus_text.get_object().unwrap()),
        JValue::from(false)
    ]).unwrap();
}


#[derive(BingusModuleTrait)]
#[add_bingus_fields]
#[bingus_module(name = "ChatSender (useless testing module)", tick_method = "tick(_env, _mappings_manager)", settings_list_fields = "[bool_setting]")]
pub struct ChatSender {
    bool_setting: (BingusSetting, &'static str, Option<[f32; 2]>),
}

impl MakeNewBingusModule for ChatSender {
    fn new() -> Self {
        Self {
            bool_setting: (BingusSetting::BoolSetting(false.into()), "testing name", None),
            __enabled_bool_setting: (BingusSetting::BoolSetting(false.into()), "enabled", None),
        }
    }
}
