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