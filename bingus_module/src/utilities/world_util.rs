use crate::crate_prelude::*;

pub fn get_world_checked<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, minecraft_client: &'a ClassMapping) -> Option<&'a ClassMapping<'a>> {
    let world = mappings_manager.get("ClientWorld").unwrap();
    apply_object!(
        world,
        {
            let check_if_null = call_method_or_get_field!(env, minecraft_client, "world", false).unwrap().l().unwrap();
            if env.is_same_object(check_if_null, JObject::null()).unwrap() {
                return None;
            } else {
                check_if_null
            }
        }
    );
    Some(world)
}

// 0: peaceful, 1: easy, 2: normal, 3: hard
pub fn get_world_difficulty<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, world: &'a ClassMapping) -> i32 {
    let client_world_properties = mappings_manager.get("ClientWorldProperties").unwrap();
    apply_object!(
        client_world_properties,
        call_method_or_get_field!(
            env,
            world,
            "clientWorldProperties",
            false
        ).unwrap().l().unwrap()
    );
    let difficulty = mappings_manager.get("Difficulty").unwrap();
    apply_object!(
        difficulty,
        call_method_or_get_field!(
            env,
            client_world_properties,
            "getDifficulty",
            false,
            &[]
        ).unwrap().l().unwrap()
    );

    if env.is_same_object(
        difficulty.get_object().unwrap(),
        call_method_or_get_field!(
            env,
            difficulty,
            "PEACEFUL",
            true
        ).unwrap().l().unwrap()
    ).unwrap() {
        0
    } else if env.is_same_object(
        difficulty.get_object().unwrap(),
        call_method_or_get_field!(
            env,
            difficulty,
            "EASY",
            true
        ).unwrap().l().unwrap()
    ).unwrap() {
        1
    } else if env.is_same_object(
        difficulty.get_object().unwrap(),
        call_method_or_get_field!(
            env,
            difficulty,
            "NORMAL",
            true
        ).unwrap().l().unwrap()
    ).unwrap() {
        2
    } else {
        3
    }
}
