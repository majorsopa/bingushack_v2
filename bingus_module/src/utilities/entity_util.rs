use crate::crate_prelude::*;

pub fn is_alive<'a>(env: JNIEnv<'a>, target: &'a ClassMapping) -> bool {
    call_method_or_get_field!(env, target, "isAlive", false, &[]).unwrap().z().unwrap()
}

pub fn get_damage_tick<'a>(env: JNIEnv<'a>, entity: &'a ClassMapping) -> i32 {
    call_method_or_get_field!(env, entity, "hurtTime", false).unwrap().i().unwrap()
}

pub fn get_entity_pos_vec3d<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, entity: &'a ClassMapping) -> &'a ClassMapping<'a> {
    let vec3d = mappings_manager.get("Vec3d").unwrap();
    apply_object!(
        vec3d,
        call_method_or_get_field!(
            env,
            entity,
            "getPos",
            false,
            &[]
        ).unwrap().l().unwrap()
    );
    vec3d
}

pub fn get_entity_pos_array<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, entity: &'a ClassMapping<'a>) -> [f64; 3] {
    let vec3d = get_entity_pos_vec3d(env, mappings_manager, entity);
    let x = call_method_or_get_field!(env, vec3d, "x", false).unwrap().d().unwrap();
    let y = call_method_or_get_field!(env, vec3d, "y", false).unwrap().d().unwrap();
    let z = call_method_or_get_field!(env, vec3d, "z", false).unwrap().d().unwrap();
    [x, y, z]
}

pub fn get_bounding_box<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, entity: &'a ClassMapping<'a>) -> &'a ClassMapping<'a> {
    let bounding_box = mappings_manager.get("Box").unwrap();
    apply_object!(
        bounding_box,
        call_method_or_get_field!(
            env,
            entity,
            "calculateBoundingBox",
            false,
            &[]
        ).unwrap().l().unwrap()
    );
    bounding_box
}

pub fn get_entity_bounding_box_minmax_array<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, minecraft_client: &'a ClassMapping<'a>, player: &'a ClassMapping<'a>, entity: &'a ClassMapping<'a>) -> [f64; 6] {
    let entity_box = get_bounding_box(env, mappings_manager, entity);
    bounding_box_minmax_array(env, mappings_manager, minecraft_client, player, entity, entity_box)
}

pub fn get_entity_id<'a>(env: JNIEnv<'a>, entity: &'a ClassMapping<'a>) -> i32 {
    call_method_or_get_field!(
        env,
        entity,
        "getId",
        false,
        &[]
    ).unwrap().i().unwrap()
}

pub fn do_attack<'a>(env: JNIEnv<'a>, minecraft_client: &'a ClassMapping<'a>) {
    let flag = call_method_or_get_field!(
        env,
        minecraft_client,
        "startAttack",
        false,
        &[]
    ).unwrap();
    call_method_or_get_field!(
        env,
        minecraft_client,
        "continueAttack",
        false,
        &[
            flag
        ]
    ).unwrap();
}
