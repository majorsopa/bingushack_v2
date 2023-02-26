use std::time::SystemTime;

use rand::Rng;

use crate::crate_prelude::*;


fn tick(totem_assist: &mut TotemAssist, env: JNIEnv, mappings_manager: &MappingsManager) {
    if totem_assist.time_since_lost_totem.is_none() {
        totem_assist.time_since_lost_totem = Some(SystemTime::now());
    }
    match totem_assist.randomly_chosen_time {
        Some(time) => {
            let time_since_lost_totem = totem_assist.time_since_lost_totem.unwrap();
            let time_since_lost_totem = time_since_lost_totem.elapsed().unwrap().as_millis();
            if time_since_lost_totem >= time {
                totem_assist.time_since_lost_totem = None;
                totem_assist.randomly_chosen_time = None;
            } else {
                return;
            }
        }
        None => {
            let new_random_delay: u128 = {
                let bounds = totem_assist.delay_setting.0.get_range();
                if bounds[0] == bounds[1] {
                    bounds[0] as u128
                } else {
                    let mut rng = rand::thread_rng();
                    rng.gen_range((bounds[0] as u128)..(bounds[1] as u128))
                }
            };
            totem_assist.randomly_chosen_time = Some(new_random_delay);
            return;
        }
    }


    let minecraft_client = get_minecraft_client(env, mappings_manager);
    let player = match get_player_checked(env, mappings_manager, minecraft_client) {
        Some(player) => player,
        None => return,
    };

    let offhand_item_id = get_raw_id_of_item_object(env, mappings_manager, get_offhand_item(env, mappings_manager, player));

    let totem_raw_id = get_raw_id_of_item(env, mappings_manager, "TOTEM_OF_UNDYING");

    let handled_screen = {
        let handled_screen = mappings_manager.get("HandledScreen").unwrap();
        let current_screen = match get_current_screen_checked(env, mappings_manager, minecraft_client) {
            Some(screen) => screen,
            None => return,
        };
        if is_instance_of(env, current_screen, handled_screen) {
            apply_object!(handled_screen, current_screen.get_object().unwrap());
            handled_screen
        } else {
            return;
        }
    };
    let focused_slot = match get_focused_slot_checked(env, mappings_manager, handled_screen) {
        Some(slot) => slot,
        None => return,
    };

    let focused_slot_stack = mappings_manager.get("ItemStack").unwrap();
    apply_object!(
        focused_slot_stack,
        call_method_or_get_field!(env, focused_slot, "getStack", false, &[]).unwrap().l().unwrap()
    );

    let focused_slot_id = get_raw_id_of_item_object(env, mappings_manager, item_from_item_stack(env, mappings_manager, focused_slot_stack));

    if focused_slot_id != totem_raw_id {
        return;
    }

    // first hotbar slot is 36-45 but for this to work it needs to be 0-9
    let focused_slot_index = {
        let actual_index = get_slot_index(env, focused_slot);
        if actual_index > 35 {
            actual_index - 36
        } else {
            actual_index
        }
    };

    let player = match get_player_checked(env, mappings_manager, minecraft_client) {
        Some(player) => player,
        None => return,
    };

    if offhand_item_id != totem_raw_id {  // if no offhand totem
        swap_slots(env, mappings_manager, minecraft_client, player, 45, focused_slot_index);
    } else {  // move to hotbar
        if !*totem_assist.fill_hotbar.0.get_bool() || focused_slot_index < 10 {
            return;
        }

        shift_click_slot(env, mappings_manager, minecraft_client, player, focused_slot_index);
    }
}


#[derive(BingusModuleTrait)]
#[add_bingus_fields]
#[bingus_module(name = "TotemAssist", tick_method = "tick(self, _env, _mappings_manager)", settings_list_fields = "[delay_setting, fill_hotbar]")]
pub struct TotemAssist {
    delay_setting: (BingusSetting, &'static str, Option<[f32; 2]>),
    fill_hotbar: (BingusSetting, &'static str, Option<[f32; 2]>),
    randomly_chosen_time: Option<u128>,
    time_since_lost_totem: Option<SystemTime>,
}

impl MakeNewBingusModule for TotemAssist {
    fn new() -> Self {
        Self {
            delay_setting: (BingusSetting::RangeSetting([30.0, 45.0].into()), "delay (ms)", Some([1.0, 120.0])),
            fill_hotbar: (BingusSetting::BoolSetting(true.into()), "fill hotbar", None),
            randomly_chosen_time: None,
            time_since_lost_totem: None,
            __enabled_bool_setting: (BingusSetting::BoolSetting(false.into()), "enabled", None),
            __keybind_setting: (BingusSetting::KeySetting(String::from("").into()), "keybind", None),
            __env: None,
            __mappings_manager: None,
            __prev_enabled: false,
        }
    }
}
