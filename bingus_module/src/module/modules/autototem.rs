use std::time::SystemTime;

use rand::Rng;
use crate::crate_prelude::*;

fn tick(autototem: &mut Autototem, env: JNIEnv, mappings_manager: &MappingsManager) {
    if autototem.time_since_lost_totem.is_none() {
        autototem.time_since_lost_totem = Some(SystemTime::now());
    }
    match autototem.randomly_chosen_time {
        Some(time) => {
            let time_since_lost_totem = autototem.time_since_lost_totem.unwrap();
            let time_since_lost_totem = time_since_lost_totem.elapsed().unwrap().as_millis();
            if time_since_lost_totem >= time {
                autototem.time_since_lost_totem = None;
                autototem.randomly_chosen_time = None;
            } else {
                return;
            }
        }
        None => {
            let new_random_delay: u128 = {
                let bounds = autototem.delay_setting.0.get_range();
                if bounds[0] == bounds[1] {
                    bounds[0] as u128
                } else {
                    let mut rng = rand::thread_rng();
                    rng.gen_range((bounds[0] as u128)..(bounds[1] as u128))
                }
            };
            autototem.randomly_chosen_time = Some(new_random_delay);
            return;
        }
    }






    let minecraft_client = get_minecraft_client(env, mappings_manager);

    let player = match get_player_checked(env, mappings_manager, minecraft_client) {
        Some(player) => player,
        None => return,
    };

    let inventory = get_inventory(env, mappings_manager, player);

    let offhand_item = get_offhand_item(env, mappings_manager, player);

    let totem_of_undying_id = get_raw_id_of_item(env, mappings_manager, "TOTEM_OF_UNDYING");



    if get_raw_id_of_item_object(env, mappings_manager, offhand_item) != totem_of_undying_id {
        let mut found_totem_slots: Vec<i32> = vec![];

        for i in 0..45 {
            let res = {
                let i_item_stack = get_inventory_slot_item_stack(env, mappings_manager, inventory, i);

                let i_item = item_from_item_stack(env, mappings_manager, i_item_stack);

                get_raw_id_of_item_object(env, mappings_manager, i_item) == totem_of_undying_id
            };
            
            if res {
                found_totem_slots.push(i);
            }
        }

        // swap totem to offhand
        if found_totem_slots.len() > 0 {
            if *autototem.hotbar_only.0.get_bool() {
                let hotbar_totem = found_totem_slots.iter().find(|&&slot| slot < 9);
                if let Some(hotbar_totem) = hotbar_totem {
                    let hotbar_totem = *hotbar_totem;

                    if autototem.hotbar_swap_prev_slot.is_none() {
                        autototem.hotbar_swap_prev_slot = Some((get_selected_slot(env, inventory), false));
                        set_selected_slot(env, inventory, hotbar_totem);
                        return;
                    } else {
                        let (prev_slot, swapped) = autototem.hotbar_swap_prev_slot.unwrap();
                        if !swapped {
                            swap_offhand_handled(env, mappings_manager, minecraft_client, player, hotbar_totem);
                            autototem.hotbar_swap_prev_slot = Some((prev_slot, true));
                            return;
                        }
                    }
                }
            } else {
                swap_offhand_handled(env, mappings_manager, minecraft_client, player, found_totem_slots[0]);
            }
        }
    }
    if autototem.hotbar_swap_prev_slot.is_some() {
        let (prev_slot, swapped) = autototem.hotbar_swap_prev_slot.unwrap();
        if swapped {
            set_selected_slot(env, inventory, prev_slot);
        }
        autototem.hotbar_swap_prev_slot = None;
    }
}

#[derive(BingusModuleTrait)]
#[add_bingus_fields]
#[bingus_module(name = "Autototem", tick_method = "tick(self, _env, _mappings_manager)", settings_list_fields = "[delay_setting, hotbar_only]")]
pub struct Autototem {
    delay_setting: (BingusSetting, &'static str, Option<[f32; 2]>),
    hotbar_only: (BingusSetting, &'static str, Option<[f32; 2]>),
    randomly_chosen_time: Option<u128>,
    time_since_lost_totem: Option<SystemTime>,
    hotbar_swap_prev_slot: Option<(i32, bool)>,
}

impl MakeNewBingusModule for Autototem {
    fn new() -> Self {
        Self {
            delay_setting: (BingusSetting::RangeSetting([100.0, 250.0].into()), "delay (ms)", Some([1.0, 1500.0])),
            hotbar_only: (BingusSetting::BoolSetting(true.into()), "hotbar only", None),
            randomly_chosen_time: None,
            time_since_lost_totem: None,
            hotbar_swap_prev_slot: None,
            __enabled_bool_setting: (BingusSetting::BoolSetting(false.into()), "enabled", None),
            __keybind_setting: (BingusSetting::KeySetting(String::from("").into()), "keybind", None),
            __env: None,
            __mappings_manager: None,
        }
    }
}
