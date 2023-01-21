use crate::crate_prelude::*;

pub fn get_minecraft_client<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager) -> &'a ClassMapping<'a> {
    let minecraft_client = mappings_manager.get("MinecraftClient").unwrap();
    apply_object!(
        minecraft_client,
        call_method_or_get_field!(env, minecraft_client, "getInstance", true, &[]).unwrap().l().unwrap()
    );
    minecraft_client
}

pub fn get_interaction_manager<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, minecraft_client: &'a ClassMapping) -> &'a ClassMapping<'a> {
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
    interaction_manager
}

pub fn facing_entity<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, minecraft_client: &'a ClassMapping) -> bool {
    let hit_result = mappings_manager.get("HitResult").unwrap();
    apply_object!(
        hit_result,
        call_method_or_get_field!(
            env,
            minecraft_client,
            "crosshairTarget",
            false
        ).unwrap().l().unwrap()
    );

    // check if the hit result is null (important)
    if env.is_same_object(hit_result.get_object().unwrap(), JObject::null()).unwrap() {
        return false;
    }
    let hit_result_type = mappings_manager.get("HitResultType").unwrap();
    apply_object!(
        hit_result_type,
        call_method_or_get_field!(
            env,
            hit_result,
            "getType",
            false,
            &[]
        ).unwrap().l().unwrap()
    );

    let entity_hit_result_field_object = call_method_or_get_field!(
        env,
        hit_result_type,
        "ENTITY",
        true
    ).unwrap().l().unwrap();

    env.is_same_object(hit_result_type.get_object().unwrap(), entity_hit_result_field_object).unwrap()
}

pub fn get_targeted_entity<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, minecraft_client: &'a ClassMapping) -> Option<&'a ClassMapping<'a>> {
    let targeted_entity = mappings_manager.get("Entity").unwrap();
    apply_object!(
        targeted_entity,
        call_method_or_get_field!(
            env,
            minecraft_client,
            "targetedEntity",
            false
        ).unwrap().l().unwrap()
    );
    if env.is_same_object(targeted_entity.get_object().unwrap(), JObject::null()).unwrap() {
        return None;
    } else {
        return Some(targeted_entity);
    }
}

pub fn get_tick_delta<'a>(env: JNIEnv<'a>, minecraft_client: &'a ClassMapping) -> f32 {
    call_method_or_get_field!(
        env,
        minecraft_client,
        "getTickDelta",
        false,
        &[]
    ).unwrap().f().unwrap()
}