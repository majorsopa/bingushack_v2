use std::time::SystemTime;

use crate::crate_prelude::*;

fn tick(triggerbot: &mut Triggerbot, env: JNIEnv, mappings_manager: &MappingsManager) {
    let minecraft_client = get_minecraft_client(env, mappings_manager);

    let player = match get_player_checked(env, mappings_manager, minecraft_client) {
        Some(player) => player,
        None => return,
    };

    let must_be_living = *triggerbot.must_be_living.0.get_bool();

    let targeted_entity = match get_targeted_entity(env, mappings_manager, minecraft_client, must_be_living) {
        Some(targeted_entity) => targeted_entity,
        None => return,
    };

    if !is_alive(env, targeted_entity) {
        return;
    }

    if *triggerbot.stop_while_using_item.0.get_bool() && is_using_item(env, player) {
        return;
    }

    if must_be_living && *triggerbot.wait_for_damage_tick.0.get_bool() && get_damage_tick(env, targeted_entity) != 0 {
        return;
    }

    if *triggerbot.crystal_dont_wait_for_cooldown.0.get_bool() {
        if !env.is_instance_of(targeted_entity.get_object().unwrap(), mappings_manager.get("EndCrystalEntity").unwrap().get_class()).unwrap() {
            if *triggerbot.wait_for_cooldown.0.get_bool() && get_attack_cooldown_progress(env, player, get_tick_delta(env, minecraft_client)) != 1.0 {
                return;
            }
        }
    } else {
        if *triggerbot.wait_for_cooldown.0.get_bool() && get_attack_cooldown_progress(env, player, get_tick_delta(env, minecraft_client)) != 1.0 {
            return;
        }
    }


    if triggerbot.last_attack.is_none() {
        triggerbot.last_attack = Some(SystemTime::now());
    } else {
        let last_attack = triggerbot.last_attack.unwrap();
        if last_attack.elapsed().unwrap().as_millis() > 50 {
            triggerbot.last_attack = None;
        } else {
            return;
        }
    }


    do_attack(env, minecraft_client);
}

#[derive(BingusModuleTrait)]
#[add_bingus_fields]
#[bingus_module(
    name = "Triggerbot",
    tick_method = "tick(self, _env, _mappings_manager)",
    settings_list_fields = "[wait_for_cooldown, crystal_dont_wait_for_cooldown, wait_for_damage_tick, stop_while_using_item, must_be_living]"
)]
pub struct Triggerbot {
    wait_for_cooldown: (BingusSetting, &'static str, Option<[f32; 2]>),
    crystal_dont_wait_for_cooldown: (BingusSetting, &'static str, Option<[f32; 2]>),
    wait_for_damage_tick: (BingusSetting, &'static str, Option<[f32; 2]>),
    stop_while_using_item: (BingusSetting, &'static str, Option<[f32; 2]>),
    must_be_living: (BingusSetting, &'static str, Option<[f32; 2]>),
    last_attack: Option<SystemTime>,
}

impl MakeNewBingusModule for Triggerbot {
    fn new() -> Self {
        Self {
            wait_for_cooldown: (BingusSetting::BoolSetting(true.into()), "wait for mob cooldowns", None),
            crystal_dont_wait_for_cooldown: (BingusSetting::BoolSetting(true.into()), "don't wait for cooldowns on crystals", None),
            wait_for_damage_tick: (BingusSetting::BoolSetting(true.into()), "wait for damage tick", None),
            stop_while_using_item: (BingusSetting::BoolSetting(true.into()), "stop while using item", None),
            must_be_living: (BingusSetting::BoolSetting(false.into()), "must be living (turn off for end crystals)", None),
            __enabled_bool_setting: (BingusSetting::BoolSetting(false.into()), "enabled", None),
            __keybind_setting: (BingusSetting::KeySetting(String::from("").into()), "keybind", None),
            last_attack: None,
            __env: None,
            __mappings_manager: None,
            __prev_enabled: false,
        }
    }
}