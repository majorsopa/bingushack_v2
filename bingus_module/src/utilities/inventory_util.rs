use crate::crate_prelude::*;

pub fn get_inventory<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, player: &'a ClassMapping) -> &'a ClassMapping<'a> {
    let inventory = mappings_manager.get("Inventory").unwrap();
    apply_object!(
        inventory,
        call_method_or_get_field!(env, player, "getInventory", false, &[]).unwrap().l().unwrap()
    );
    inventory
}

pub fn click_slot<'a>(env: JNIEnv<'a>, player: &'a ClassMapping, interaction_manager: &'a ClassMapping, sync_id: i32, slot: i32, slot_action_type: JValue) {
    call_method_or_get_field!(
        env,
        interaction_manager,
        "clickSlot",
        false,
        &[
            JValue::from(sync_id),
            JValue::from(slot),
            JValue::from(0),
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
    let field_name = "selectedSlot";
    let slot_type = inventory.get_field(field_name, false).unwrap().get_sig();
    env.set_field(inventory.get_object().unwrap(), field_name, slot_type, JValue::from(slot)).unwrap();
}

pub fn swap_offhand<'a>(env: JNIEnv<'a>, player: &'a ClassMapping) {
    call_method_or_get_field!(
        env,
        player,
        "swapHandStacks",
        false,
        &[]
    ).unwrap();
}