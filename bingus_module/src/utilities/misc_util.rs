use crate::crate_prelude::*;

pub fn is_instance_of<'a>(env: JNIEnv<'a>, object: &'a ClassMapping<'a>, class: &'a ClassMapping<'a>) -> bool {
    env.is_instance_of(object.get_object().unwrap(), class.get_class()).unwrap()
}

pub fn get_simple_option<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, simple_option: &'a ClassMapping) -> &'a ClassMapping<'a> {
    let value = mappings_manager.get("Object").unwrap();
    apply_object!(
        value,
        call_method_or_get_field!(
            env,
            simple_option,
            "value",
            false
        ).unwrap().l().unwrap()
    );
    value
}

pub fn set_simple_option<'a>(env: JNIEnv<'a>, simple_option: &'a ClassMapping, value: &'a ClassMapping<'a>) {
    let sig_holder = simple_option.get_field("value", false).unwrap();
    let name = sig_holder.get_name();
    let ty = sig_holder.get_sig();
    env.set_field(
        simple_option.get_object().unwrap(),
        name,
        ty,
        JValue::from(value.get_object().unwrap())
    ).unwrap();
}

pub fn has_next_java_iterator<'a>(env: JNIEnv<'a>, iterator: &'a ClassMapping<'a>) -> bool {
    call_method_or_get_field!(
        env,
        iterator,
        "hasNext",
        false,
        &[]
    ).unwrap().z().unwrap()
}

pub fn get_next_java_iterator<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, iterator: &'a ClassMapping<'a>) -> &'a ClassMapping<'a> {
    let value = mappings_manager.get("Object").unwrap();
    apply_object!(
        value,
        call_method_or_get_field!(
            env,
            iterator,
            "next",
            false,
            &[]
        ).unwrap().l().unwrap()
    );
    value
}

pub fn get_next_java_iterator_checked<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, iterator: &'a ClassMapping<'a>) -> Option<&'a ClassMapping<'a>> {
    if has_next_java_iterator(env, iterator) {
        Some(get_next_java_iterator(env, mappings_manager, iterator))
    } else {
        None
    }
}

pub fn java_iterable_to_iterator<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, iterable: &'a ClassMapping<'a>) -> &'a ClassMapping<'a> {
    let iterator = mappings_manager.get("Iterator").unwrap();
    apply_object!(
        iterator,
        call_method_or_get_field!(
            env,
            iterable,
            "iterator",
            false,
            &[]
        ).unwrap().l().unwrap()
    );
    iterator
}

pub fn bounding_box_minmax_array<'a>(
    env: JNIEnv<'a>,
    mappings_manager: &'a MappingsManager,
    minecraft_client: &'a ClassMapping<'a>,
    player: &'a ClassMapping<'a>,
    entity: &'a ClassMapping<'a>,
    bounding_box: &'a ClassMapping<'a>
) -> [f64; 6] {
    let last_render_pos_x = call_method_or_get_field!(
        env,
        entity,
        "lastRenderX",
        false
    ).unwrap().d().unwrap();
    let last_render_pos_y = call_method_or_get_field!(
        env,
        entity,
        "lastRenderY",
        false
    ).unwrap().d().unwrap();
    let last_render_pos_z = call_method_or_get_field!(
        env,
        entity,
        "lastRenderZ",
        false
    ).unwrap().d().unwrap();
    let [pos_x, pos_y, pos_z] = get_entity_pos_array(env, mappings_manager, entity);


    let mut min_x = call_method_or_get_field!(
        env,
        bounding_box,
        "minX",
        false
    ).unwrap().d().unwrap();
    let mut min_y = call_method_or_get_field!(
        env,
        bounding_box,
        "minY",
        false
    ).unwrap().d().unwrap();
    let mut min_z = call_method_or_get_field!(
        env,
        bounding_box,
        "minZ",
        false
    ).unwrap().d().unwrap();
    let mut max_x = call_method_or_get_field!(
        env,
        bounding_box,
        "maxX",
        false
    ).unwrap().d().unwrap();
    let mut max_y = call_method_or_get_field!(
        env,
        bounding_box,
        "maxY",
        false
    ).unwrap().d().unwrap();
    let mut max_z = call_method_or_get_field!(
        env,
        bounding_box,
        "maxZ",
        false
    ).unwrap().d().unwrap();

    let partial_tick = {
        let render_tick_counter = mappings_manager.get("RenderTickCounter").unwrap();
        apply_object!(
            render_tick_counter,
            call_method_or_get_field!(
                env,
                minecraft_client,
                "renderTickCounter",
                false
            ).unwrap().l().unwrap()
        );
        call_method_or_get_field!(
            env,
            render_tick_counter,
            "partialTick",
            false
        ).unwrap().f().unwrap()
    } as f64;

    let [player_x, player_y, player_z] = get_player_pos(env, mappings_manager, player);

    min_x = min_x - pos_x + (last_render_pos_x + (pos_x - last_render_pos_x) * partial_tick) - player_x;
    min_y = min_y - pos_y + (last_render_pos_y + (pos_y - last_render_pos_y) * partial_tick) - player_y;
    min_z = min_z - pos_z + (last_render_pos_z + (pos_z - last_render_pos_z) * partial_tick) - player_z;
    max_x = max_x - pos_x + (last_render_pos_x + (pos_x - last_render_pos_x) * partial_tick) - player_x;
    max_y = max_y - pos_y + (last_render_pos_y + (pos_y - last_render_pos_y) * partial_tick) - player_y;
    max_z = max_z - pos_z + (last_render_pos_z + (pos_z - last_render_pos_z) * partial_tick) - player_z;

    [min_x, min_y, min_z, max_x, max_y, max_z]
}

pub fn make_raycast_miss_lambda<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, raycast_context: &'a ClassMapping<'a>) -> &'a ClassMapping<'a> {
    let function = mappings_manager.get("Function").unwrap();
    apply_object!(
        function,
        call_method_or_get_field!(
            env,
            function,
            "identity",
            true,
            &[]
        ).unwrap().l().unwrap()
    );
    // function is an object but it doesn't do anything yet
    // time to apply a closure

    let closure_to_apply = Box::new(|raycast_context_object| -> jni::sys::jobject {

    });

    function
}