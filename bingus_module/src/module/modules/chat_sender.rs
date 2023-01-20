use jni::objects::JValue;
use mappings_macro::{apply_object, call_method_or_get_field};

use crate::crate_prelude::*;


fn tick(env: JNIEnv, mappings_manager: &MappingsManager) {
    let minecraft_client = mappings_manager.get("MinecraftClient").unwrap();
    apply_object!(
        minecraft_client,
        call_method_or_get_field!(env, minecraft_client, "getInstance", true, &[]).unwrap().l().unwrap()
    );

    let in_game_hud = mappings_manager.get("InGameHud").unwrap();
    apply_object!(
        in_game_hud,
        call_method_or_get_field!(env, minecraft_client, "inGameHud", false).unwrap().l().unwrap()
    );

    let chat_hud = mappings_manager.get("ChatHud").unwrap();
    apply_object!(
        chat_hud,
        call_method_or_get_field!(env, in_game_hud, "chatHud", false, &[]).unwrap().l().unwrap()
    );

    let bingus_text = mappings_manager.get("Text").unwrap();
    apply_object!(
        bingus_text,
        call_method_or_get_field!(env, bingus_text, "of", true, &[JValue::from(env.new_string("bingus").unwrap())]).unwrap().l().unwrap()
    );

    call_method_or_get_field!(env, chat_hud, "addMessage", false, &[
        JValue::from(bingus_text.get_object().unwrap()),
        JValue::Void,
        JValue::from(200),
        JValue::Void,
        JValue::from(false),
        ]).unwrap();
}


#[derive(BingusModuleTrait)]
#[add_bingus_fields]
#[bingus_module(name = "ChatSender", tick_method = "tick(_env, _mappings_manager)", settings_list_fields = "[bool_setting]")]
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
