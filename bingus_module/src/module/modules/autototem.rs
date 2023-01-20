use jni::objects::JValue;
use crate::crate_prelude::*;
use mappings_macro::{apply_object, call_method_or_get_field};

fn tick(env: JNIEnv, mappings_manager: &MappingsManager) {
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
#[bingus_module(name = "Autototem", tick_method = "tick(_env, _mappings_manager)")]
pub struct Autototem {
    
}

impl MakeNewBingusModule for Autototem {
    fn new() -> Self {
        Self {
            __enabled_bool_setting: (BingusSetting::BoolSetting(false.into()), "enabled"),
        }
    }
}
