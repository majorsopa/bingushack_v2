use crate::crate_prelude::*;

fn tick(env: JNIEnv, mappings_manager: &MappingsManager) {
    let minecraft_client = get_minecraft_client(env, mappings_manager);

    // debug
    let player = match get_player_checked(env, mappings_manager, minecraft_client) {
        Some(player) => player,
        None => return,
    };

    let hit_result = mappings_manager.get("HitResult").unwrap();
    apply_object!(
        hit_result,
        call_method_or_get_field!(
            env,
            minecraft_client,
            "crosshairTarget",
            false
        ).unwrap().l().unwrap()
    );

    // check if the hit result is null (important)
    if env.is_same_object(hit_result.get_object().unwrap(), JObject::null()).unwrap() {
        return;
    }
    let hit_result_type = mappings_manager.get("HitResultType").unwrap();
    apply_object!(
        hit_result_type,
        call_method_or_get_field!(
            env,
            hit_result,
            "getType",
            false,
            &[]
        ).unwrap().l().unwrap()
    );

    let entity_hit_result_field_object = call_method_or_get_field!(
        env,
        hit_result_type,
        "ENTITY",
        true
    ).unwrap().l().unwrap();

    if !env.is_same_object(hit_result_type.get_object().unwrap(), entity_hit_result_field_object).unwrap() {
        send_chat_message(env, player, make_minecraft_text_object(env, mappings_manager, "not entity"));
    } else {
        send_chat_message(env, player, make_minecraft_text_object(env, mappings_manager, "yes entity"));
    }

    
}

#[derive(BingusModuleTrait)]
#[add_bingus_fields]
#[bingus_module(name = "Triggerbot", tick_method = "tick(_env, _mappings_manager)")]
pub struct Triggerbot {

}

impl MakeNewBingusModule for Triggerbot {
    fn new() -> Self {
        Self {
            __enabled_bool_setting: (BingusSetting::BoolSetting(false.into()), "enabled", None),
        }
    }
}
