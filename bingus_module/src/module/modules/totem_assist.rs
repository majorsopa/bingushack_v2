use crate::crate_prelude::*;


fn tick(env: JNIEnv, mappings_manager: &MappingsManager) {
    let minecraft_client = get_minecraft_client(env, mappings_manager);
    let player = match get_player_checked(env, mappings_manager, minecraft_client) {
        Some(player) => player,
        None => return,
    };

    let offhand_item_id = get_raw_id_of_item_object(env, mappings_manager, get_offhand_item(env, mappings_manager, player));

    let totem_raw_id = get_raw_id_of_item(env, mappings_manager, "TOTEM_OF_UNDYING");

    if offhand_item_id == totem_raw_id {
        return;
    }


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

    swap_slots(env, mappings_manager, minecraft_client, player, 45, focused_slot_index);
}


#[derive(BingusModuleTrait)]
#[add_bingus_fields]
#[bingus_module(name = "TotemAssist", tick_method = "tick(_env, _mappings_manager)")]
pub struct TotemAssist {

}

impl MakeNewBingusModule for TotemAssist {
    fn new() -> Self {
        Self {
            __enabled_bool_setting: (BingusSetting::BoolSetting(false.into()), "enabled", None),
            __keybind_setting: (BingusSetting::KeySetting(String::from("").into()), "keybind", None),
        }
    }
}
