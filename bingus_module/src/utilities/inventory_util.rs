use crate::crate_prelude::*;

pub fn get_inventory<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, player: &'a ClassMapping) -> &'a ClassMapping<'a> {
    let inventory = mappings_manager.get("Inventory").unwrap();
    apply_object!(
        inventory,
        call_method_or_get_field!(env, player, "getInventory", false, &[]).unwrap().l().unwrap()
    );
    inventory
}

pub fn click_slot<'a>(env: JNIEnv<'a>, player: &'a ClassMapping, interaction_manager: &'a ClassMapping, sync_id: i32, slot: i32, button: i32, slot_action_type: JValue) {
    call_method_or_get_field!(
        env,
        interaction_manager,
        "clickSlot",
        false,
        &[
            JValue::from(sync_id),
            JValue::from(slot),
            JValue::from(button),
            slot_action_type,
            JValue::from(player.get_object().unwrap())
        ]
    ).unwrap();
}

pub fn get_selected_slot<'a>(env: JNIEnv<'a>, inventory: &'a ClassMapping) -> i32 {
    call_method_or_get_field!(
        env,
        inventory,
        "selectedSlot",
        false
    ).unwrap().i().unwrap()
}

pub fn set_selected_slot<'a>(env: JNIEnv<'a>, inventory: &'a ClassMapping, slot: i32) {
    let field_mapping = inventory.get_field("selectedSlot", false).unwrap();
    env.set_field(inventory.get_object().unwrap(), field_mapping.get_name(), field_mapping.get_sig(), JValue::from(slot)).unwrap();
}

pub fn swap_slots<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, minecraft_client: &'a ClassMapping, player: &'a ClassMapping, to_index: i32, from_index: i32) {
    let interaction_manager = get_interaction_manager(env, mappings_manager, minecraft_client);

    let current_screen_handler = get_screen_handler(env, mappings_manager, player);

    let sync_id = get_sync_id(env, current_screen_handler);

    let pickup_slot_action = JValue::from(
        call_method_or_get_field!(
            env,
            mappings_manager.get("SlotActionType").unwrap(),
            "SWAP",
            true
        ).unwrap()
    );


    click_slot(env, player, interaction_manager, sync_id, to_index, from_index, pickup_slot_action);
}

pub fn swap_offhand_handled<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, minecraft_client: &'a ClassMapping, player: &'a ClassMapping, slot: i32) {
    let interaction_manager = get_interaction_manager(env, mappings_manager, minecraft_client);

    let current_screen_handler = get_screen_handler(env, mappings_manager, player);

    let sync_id = get_sync_id(env, current_screen_handler);

    let pickup_slot_action = call_method_or_get_field!(
        env,
        mappings_manager.get("SlotActionType").unwrap(),
        if slot < 9 { "SWAP" } else { "PICKUP" },
        true
    ).unwrap();


    if slot < 9 {
        click_slot(env, player, interaction_manager, sync_id, 45, slot, pickup_slot_action);
    } else {
        // pick up
        click_slot(env, player, interaction_manager, sync_id, slot, 0, pickup_slot_action);

        // put down
        click_slot(env, player, interaction_manager, sync_id, 45, 0, pickup_slot_action);
    }
}

pub fn get_focused_slot_checked<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, handled_screen: &'a ClassMapping) -> Option<&'a ClassMapping<'a>> {
    let slot = mappings_manager.get("Slot").unwrap();
    let focused_slot = call_method_or_get_field!(env, handled_screen, "focusedSlot", false).unwrap().l().unwrap();
    if env.is_same_object(focused_slot, JObject::null()).unwrap() {
        return None;
    } else {
        apply_object!(slot, focused_slot);
        return Some(slot);
    }
}

pub fn get_slot_index<'a>(env: JNIEnv<'a>, slot: &'a ClassMapping) -> i32 {
    call_method_or_get_field!(env, slot, "index", false).unwrap().i().unwrap()
}