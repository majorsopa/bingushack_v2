use crate::crate_prelude::*;

pub fn make_minecraft_text_object<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, text: &str) -> &'a ClassMapping<'a> {
    let minecraft_text = mappings_manager.get("Text").unwrap();
    apply_object!(
        minecraft_text,
        call_method_or_get_field!(env, minecraft_text, "of", true, &[JValue::from(env.new_string(text).unwrap())]).unwrap().l().unwrap()
    );
    minecraft_text
}

pub fn send_chat_message_mc_text<'a>(env: JNIEnv<'a>, player: &'a ClassMapping, message: &'a ClassMapping) {
    call_method_or_get_field!(env, player, "displayClientMessage", false, &[
        JValue::from(message.get_object().unwrap()),
        JValue::from(false)
    ]).unwrap();
}

pub fn send_chat_message<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, player: &'a ClassMapping, message: &str) {
    call_method_or_get_field!(env, player, "displayClientMessage", false, &[
        JValue::from(make_minecraft_text_object(env, mappings_manager, message).get_object().unwrap()),
        JValue::from(false)
    ]).unwrap();
}
