use jni::objects::JValue;
use mappings_macro::{apply_object, call_method_or_get_field};

use crate::crate_prelude::*;


fn tick(env: JNIEnv, mappings_manager: &MappingsManager) {
    let minecraft_client = mappings_manager.get("MinecraftClient").unwrap();
    apply_object!(
        minecraft_client,
        call_method_or_get_field!(env, minecraft_client, "getInstance", true, &[]).unwrap().l().unwrap()
    );

    let player = mappings_manager.get("PlayerEntity").unwrap();
    apply_object!(
        player,
        {
            let check_if_null = call_method_or_get_field!(env, minecraft_client, "player", false).unwrap().l().unwrap();
            if env.is_same_object(check_if_null, JObject::null()).unwrap() {
                return;
            } else {
                check_if_null
            }
        }
    );

    let bingus_text = mappings_manager.get("Text").unwrap();
    apply_object!(
        bingus_text,
        call_method_or_get_field!(env, bingus_text, "of", true, &[JValue::from(env.new_string("bingus").unwrap())]).unwrap().l().unwrap()
    );

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
