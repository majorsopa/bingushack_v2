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
                let mut rng = rand::thread_rng();
                rng.gen_range((bounds[0] as u128)..(bounds[1] as u128))
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
        let mut found_totem_slot: Option<i32> = None;

        for i in 9..45 {
            let res = {
                let i_item_stack = get_inventory_slot_item_stack(env, mappings_manager, inventory, i);

                let i_item = item_from_item_stack(env, mappings_manager, i_item_stack);

                get_raw_id_of_item_object(env, mappings_manager, i_item) == totem_of_undying_id
            };
            
            if res {
                found_totem_slot = Some(i);
                break;
            }
        }

        // swap totem to offhand
        if let Some(found_totem_slot) = found_totem_slot {
            let interaction_manager = get_interaction_manager(env, mappings_manager, minecraft_client);

            let current_screen_handler = get_screen_handler(env, mappings_manager, player);

            let sync_id = get_sync_id(env, current_screen_handler);

            let pickup_slot_action = call_method_or_get_field!(
                env,
                mappings_manager.get("SlotActionType").unwrap(),
                "PICKUP",
                true
            ).unwrap();

            // call clickSlot
            // pick up
            click_slot(env, player, interaction_manager, sync_id, found_totem_slot, pickup_slot_action);

            // put down
            click_slot(env, player, interaction_manager, sync_id, 45, pickup_slot_action);
        }
    }
}

#[derive(BingusModuleTrait)]
#[add_bingus_fields]
#[bingus_module(name = "Autototem", tick_method = "tick(self, _env, _mappings_manager)", settings_list_fields = "[delay_setting]")]
pub struct Autototem {
    delay_setting: (BingusSetting, &'static str, Option<[f32; 2]>),
    randomly_chosen_time: Option<u128>,
    time_since_lost_totem: Option<SystemTime>,
}

impl MakeNewBingusModule for Autototem {
    fn new() -> Self {
        Self {
            delay_setting: (BingusSetting::RangeSetting([500.0, 1000.0].into()), "delay (ms)", Some([0.0, 5000.0])),
            randomly_chosen_time: None,
            time_since_lost_totem: None,
            __enabled_bool_setting: (BingusSetting::BoolSetting(false.into()), "enabled", None),
        }
    }
}
