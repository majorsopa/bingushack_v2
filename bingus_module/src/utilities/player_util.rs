use crate::crate_prelude::*;

pub fn get_player_checked<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, minecraft_client: &'a ClassMapping) -> Option<&'a ClassMapping<'a>> {
    let player = mappings_manager.get("PlayerEntity").unwrap();
    apply_object!(
        player,
        {
            let check_if_null = call_method_or_get_field!(env, minecraft_client, "player", false).unwrap().l().unwrap();
            if env.is_same_object(check_if_null, JObject::null()).unwrap() {
                return None;
            } else {
                check_if_null
            }
        }
    );
    Some(player)
}

pub fn get_offhand_item<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, player: &'a ClassMapping) -> &'a ClassMapping<'a> {
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
    offhand_item
}

pub fn get_inventory_slot_item_stack<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, inventory: &'a ClassMapping, i: i32) -> &'a ClassMapping<'a> {
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
    i_item_stack
}

pub fn get_screen_handler<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, player: &'a ClassMapping) -> &'a ClassMapping<'a> {
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
    current_screen_handler
}

pub fn get_sync_id<'a>(env: JNIEnv<'a>, screen_handler: &'a ClassMapping) -> i32 {
    call_method_or_get_field!(
        env,
        screen_handler,
        "syncId",
        false
    ).unwrap().i().unwrap()
}

pub fn get_attack_cooldown_progress<'a>(env: JNIEnv<'a>, player: &'a ClassMapping, tick_delta: f32) -> f32 {
    call_method_or_get_field!(
        env,
        player,
        "getAttackCooldownProgress",
        false,
        &[JValue::from(tick_delta)]
    ).unwrap().f().unwrap()
}

pub fn is_using_item<'a>(env: JNIEnv<'a>, player: &'a ClassMapping) -> bool {
    call_method_or_get_field!(
        env,
        player,
        "isUsingItem",
        false,
        &[]
    ).unwrap().z().unwrap()
}

pub fn swing_hand<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, player: &'a ClassMapping, main_hand: bool) {
    call_method_or_get_field!(
        env,
        player,
        "swingHand",
        false,
        &[
            call_method_or_get_field!(
                env,
                mappings_manager.get("Hand").unwrap(),
                if main_hand { "MAIN_HAND" } else { "OFF_HAND" },
                true
            ).unwrap(),
            JValue::from(false),
        ]
    ).unwrap();
}

pub fn get_player_pos<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, player: &'a ClassMapping) -> [f64; 3] {
    let player_entity = mappings_manager.get("Entity").unwrap();
    apply_object!(
        player_entity,
        player.get_object().unwrap()
    );

    get_entity_pos_array(env, mappings_manager, player_entity)
}