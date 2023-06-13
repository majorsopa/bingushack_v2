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


pub fn get_targeted_entity<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, minecraft_client: &'a ClassMapping, must_be_living: bool) -> Option<&'a ClassMapping<'a>> {
    // make sure the object in instanceof is not null
    let ray_trace = mappings_manager.get("HitResult").unwrap();
    apply_object!(
        ray_trace,
        call_method_or_get_field!(
            env,
            minecraft_client,
            "crosshairTarget",
            false
        ).unwrap().l().unwrap()
    );

    if env.is_same_object(ray_trace.get_object().unwrap(), JObject::null()).unwrap() {
        return None;
    }

    if !env.is_instance_of(ray_trace.get_object().unwrap(), mappings_manager.get("EntityHitResult").unwrap().get_class()).unwrap() {
        return None;
    }

    let ray_trace_entity = mappings_manager.get("EntityHitResult").unwrap();
    apply_object!(
        ray_trace_entity,
        ray_trace.get_object().unwrap()
    );

    let entity = mappings_manager.get("Entity").unwrap();
    apply_object!(
        entity,
        call_method_or_get_field!(
            env,
            ray_trace_entity,
            "getEntity",
            false,
            &[]
        ).unwrap().l().unwrap()
    );

    if env.is_same_object(entity.get_object().unwrap(), JObject::null()).unwrap() {
        return None;
    }

    if must_be_living {
        if env.is_instance_of(entity.get_object().unwrap(), mappings_manager.get("LivingEntity").unwrap().get_class()).unwrap() {
            return Some(entity);
        } else {
            return None;
        }
    } else {
        return Some(entity);
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

pub fn get_current_screen_checked<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, minecraft_client: &'a ClassMapping) -> Option<&'a ClassMapping<'a>> {
    let screen = mappings_manager.get("Screen").unwrap();
    apply_object!(
        screen,
        {
            let check_if_null = call_method_or_get_field!(env, minecraft_client, "currentScreen", false).unwrap().l().unwrap();
            if env.is_same_object(check_if_null, JObject::null()).unwrap() {
                return None;
            } else {
                check_if_null
            }
        }
    );
    Some(screen)
}

pub fn get_game_options<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, minecraft_client: &'a ClassMapping) -> &'a ClassMapping<'a> {
    let game_options = mappings_manager.get("GameOptions").unwrap();
    apply_object!(
        game_options,
        call_method_or_get_field!(
            env,
            minecraft_client,
            "options",
            false
        ).unwrap().l().unwrap()
    );
    game_options
}

pub fn get_fov(env: JNIEnv, mappings_manager: &MappingsManager, game_options: &ClassMapping) -> i32 {
    let prev_fov_option = mappings_manager.get("SimpleOption").unwrap();
    apply_object!(
        prev_fov_option,
        call_method_or_get_field!(
            env,
            game_options,
            "fov",
            false
        ).unwrap().l().unwrap()
    );
    let prev_fov_integer = get_simple_option(env, mappings_manager, prev_fov_option);
    int_object_to_int(env, mappings_manager, prev_fov_integer)
}

pub fn set_fov<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, minecraft_client: &'a ClassMapping, fov: i32) {
    let game_options = get_game_options(env, mappings_manager, minecraft_client);

    let fov_option = mappings_manager.get("SimpleOption").unwrap();
    apply_object!(
        fov_option,
        call_method_or_get_field!(
            env,
            game_options,
            "fov",
            false
        ).unwrap().l().unwrap()
    );
    set_simple_option(env, fov_option, int_to_int_object(env, mappings_manager, fov));
}

pub fn int_object_to_int<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, int_object: &'a ClassMapping<'a>) -> i32 {
    let casted_int_object = mappings_manager.get("Integer").unwrap();
    apply_object!(
        casted_int_object,
        int_object.get_object().unwrap()
    );
    call_method_or_get_field!(
        env,
        casted_int_object,
        "intValue",
        false,
        &[]
    ).unwrap().i().unwrap()
}

pub fn int_to_int_object<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, int: i32) -> &'a ClassMapping<'a> {
    let integer = mappings_manager.get("Integer").unwrap();
    apply_object!(
        integer,
        call_method_or_get_field!(
            env,
            integer,
            "valueOf",
            true,
            &[JValue::from(int)]
        ).unwrap().l().unwrap()
    );
    integer
}