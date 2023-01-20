use std::time::SystemTime;

use jni::objects::JValue;
use rand::Rng;
use crate::crate_prelude::*;
use mappings_macro::{apply_object, call_method_or_get_field};

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






    let minecraft_client = mappings_manager.get("MinecraftClient").unwrap();
    apply_object!(
        minecraft_client,
        call_method_or_get_field!(env, minecraft_client, "getInstance", true, &[]).unwrap().l().unwrap()
    );

    let player = mappings_manager.get("PlayerEntity").unwrap();
    apply_object!(
        player,
        call_method_or_get_field!(env, minecraft_client, "player", false).unwrap().l().unwrap()
    );

    let inventory = mappings_manager.get("Inventory").unwrap();
    apply_object!(
        inventory,
        call_method_or_get_field!(env, player, "getInventory", false, &[]).unwrap().l().unwrap()
    );

    let offhand_item = mappings_manager.get("Item").unwrap();
    {
        let item_stack = mappings_manager.get("ItemStack").unwrap();
        apply_object!(
            item_stack,
            call_method_or_get_field!(env, player, "getOffHandStack", false, &[]).unwrap().l().unwrap()
        );
        apply_object!(
            offhand_item,
            call_method_or_get_field!(env, item_stack, "getItem", false, &[]).unwrap().l().unwrap()
        );
    }

    let totem_of_undying_id = {
        let totem_of_undying = mappings_manager.get("Items").unwrap();
        apply_object!(
            totem_of_undying,
            call_method_or_get_field!(env, totem_of_undying, "TOTEM_OF_UNDYING", true).unwrap().l().unwrap()
        );

        call_method_or_get_field!(
            env,
            offhand_item,
            "getRawId",
            true,
            &[JValue::from(totem_of_undying.get_object().unwrap())]
        )
        .unwrap()
        .i()
        .unwrap()
    };

    let offhand_is_totem = call_method_or_get_field!(
        env,
        offhand_item,
        "getRawId",
        true,
        &[JValue::from(offhand_item.get_object().unwrap())]
    ).unwrap().i().unwrap() == totem_of_undying_id;



    if !offhand_is_totem {
        let mut found_totem_slot: Option<i32> = None;

        for i in 9..45 {
            let res = {
                let i_item_stack = mappings_manager.get("ItemStack").unwrap();
                // call getStack(i) on inventory then getItem on the result then getRawId on the result of that
                apply_object!(
                    i_item_stack,
                    call_method_or_get_field!(
                        env,
                        inventory,
                        "getStack",
                        false,
                        &[JValue::from(i)]
                    ).unwrap().l().unwrap()
                );

                let i_item = mappings_manager.get("Item").unwrap();
                apply_object!(
                    i_item,
                    call_method_or_get_field!(env, i_item_stack, "getItem", false, &[]).unwrap().l().unwrap()
                );

                call_method_or_get_field!(
                    env,
                    i_item,
                    "getRawId",
                    true,
                    &[JValue::from(i_item.get_object().unwrap())]
                ).unwrap().i().unwrap() == totem_of_undying_id
            };
            
            if res {
                found_totem_slot = Some(i);
                break;
            }
        }

        // swap totem to offhand
        if let Some(found_totem_slot) = found_totem_slot {
            let interaction_manager = mappings_manager.get("InteractionManager").unwrap();
            apply_object!(
                interaction_manager,
                call_method_or_get_field!(
                    env,
                    minecraft_client,
                    "interactionManager",
                    false
                ).unwrap().l().unwrap()
            );

            let current_screen_handler = mappings_manager.get("ScreenHandler").unwrap();
            apply_object!(
                current_screen_handler,
                call_method_or_get_field!(
                    env,
                    player,
                    "currentScreenHandler",
                    false
                ).unwrap().l().unwrap()
            );

            let sync_id = call_method_or_get_field!(
                env,
                current_screen_handler,
                "syncId",
                false
            ).unwrap().i().unwrap();

            let pickup_slot_action = call_method_or_get_field!(
                env,
                mappings_manager.get("SlotActionType").unwrap(),
                "PICKUP",
                true
            ).unwrap();

            // call clickSlot
            // pick up
            call_method_or_get_field!(
                env,
                interaction_manager,
                "clickSlot",
                false,
                &[
                    JValue::from(sync_id),
                    JValue::from(found_totem_slot),
                    JValue::from(0),
                    pickup_slot_action,
                    JValue::from(player.get_object().unwrap()),
                ]
            ).unwrap();

            // put down
            call_method_or_get_field!(
                env,
                interaction_manager,
                "clickSlot",
                false,
                &[
                    JValue::from(sync_id),
                    JValue::from(45),  // 45 is offhand slot index
                    JValue::from(0),
                    pickup_slot_action,
                    JValue::from(player.get_object().unwrap()),
                ]
            ).unwrap();
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
