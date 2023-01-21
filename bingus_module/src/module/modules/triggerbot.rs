use crate::crate_prelude::*;

fn tick(triggerbot: &mut Triggerbot, env: JNIEnv, mappings_manager: &MappingsManager) {
    let minecraft_client = get_minecraft_client(env, mappings_manager);

    let player = match get_player_checked(env, mappings_manager, minecraft_client) {
        Some(player) => player,
        None => return,
    };



    if !facing_entity(env, mappings_manager, minecraft_client) {
        return;
    }

    if triggerbot.wait_for_cooldown.0.get_bool() && get_attack_cooldown_progress(env, player, get_tick_delta(env, minecraft_client)) != 1.0 {
        return;
    }

    if triggerbot.stop_while_using_item.0.get_bool() && is_using_item(env, player) {
        return;
    }



    call_method_or_get_field!(
        env,
        minecraft_client,
        "doAttack",
        false,
        &[]
    ).unwrap();

    let world = mappings_manager.get("ClientWorld").unwrap();
    apply_object!(
        world,
        call_method_or_get_field!(
            env,
            minecraft_client,
            "world",
            false
        ).unwrap().l().unwrap()
    );

    
}

#[derive(BingusModuleTrait)]
#[add_bingus_fields]
#[bingus_module(name = "Triggerbot", tick_method = "tick(self, _env, _mappings_manager)", settings_list_fields = "[wait_for_cooldown, stop_while_using_item]")]
pub struct Triggerbot {
    wait_for_cooldown: (BingusSetting, &'static str, Option<[f32; 2]>),  // need to add a small cooldown or else it crashes when you look at an entity for too long lol
    stop_while_using_item: (BingusSetting, &'static str, Option<[f32; 2]>),
}

impl MakeNewBingusModule for Triggerbot {
    fn new() -> Self {
        Self {
            wait_for_cooldown: (BingusSetting::BoolSetting(true.into()), "wait for cooldown", None),
            stop_while_using_item: (BingusSetting::BoolSetting(true.into()), "stop while using item", None),
            __enabled_bool_setting: (BingusSetting::BoolSetting(false.into()), "enabled", None),
        }
    }
}
