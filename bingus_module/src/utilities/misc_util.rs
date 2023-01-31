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

pub fn raycast_replacement<'a>(
    env: JNIEnv<'a>,
    mappings_manager: &'a MappingsManager,
    world: &'a ClassMapping<'a>,
    start_vec3d: &'a ClassMapping<'a>,
    end_vec3d: &'a ClassMapping<'a>,
    raycast_context: &'a ClassMapping<'a>,
    obby_pos: &'a ClassMapping<'a>,
    ignore_terrain: bool
) -> &'a ClassMapping<'a> {
    let hit_closure = |raycast_context: &'a ClassMapping<'a>, block_pos: &'a ClassMapping<'a>| {
        let block_state = mappings_manager.get("BlockState").unwrap();
        if env.is_same_object(
            obby_pos.get_object().unwrap(),
            block_pos.get_object().unwrap()
        ).unwrap() {
            let obsidian_block = mappings_manager.get("Block").unwrap();
            apply_object!(
                obsidian_block,
                call_method_or_get_field!(
                    env,
                    mappings_manager.get("Blocks").unwrap(),
                    "OBSIDIAN",
                    true
                ).unwrap().l().unwrap()
            );
            let obby_default_state = call_method_or_get_field!(
                env,
                obsidian_block,
                "getDefaultState",
                false,
                &[]
            ).unwrap().l().unwrap();
            apply_object!(
                block_state,
                obby_default_state
            );
        } else {
            let world = mappings_manager.get("World").unwrap();
            apply_object!(
                world,
                call_method_or_get_field!(
                    env,
                    raycast_context,
                    "world",
                    false
                ).unwrap().l().unwrap()
            );
            let block = call_method_or_get_field!(
                env,
                world,
                "getBlockState",
                false,
                &[
                    JValue::Object(block_pos.get_object().unwrap())
                ]
            ).unwrap().l().unwrap();
            apply_object!(
                block_state,
                block
            );

            let block = mappings_manager.get("Block").unwrap();
            apply_object!(
                block,
                call_method_or_get_field!(
                    env,
                    block_state,
                    "getBlock",
                    false,
                    &[]
                ).unwrap().l().unwrap()
            );
            let blast_resistance = call_method_or_get_field!(
                env,
                block,
                "getBlastResistance",
                false,
                &[]
            ).unwrap().f().unwrap();

            if blast_resistance < 600.0 && ignore_terrain {
                let air_block = mappings_manager.get("Block").unwrap();
                apply_object!(
                    air_block,
                    call_method_or_get_field!(
                        env,
                        mappings_manager.get("Blocks").unwrap(),
                        "AIR",
                        true
                    ).unwrap().l().unwrap()
                );
                let air_default_state = call_method_or_get_field!(
                    env,
                    air_block,
                    "getDefaultState",
                    false,
                    &[]
                ).unwrap().l().unwrap();
                apply_object!(
                    block_state,
                    air_default_state
                );
            }
        }

        let raycast_start = mappings_manager.get("Vec3d").unwrap();
        apply_object!(
            raycast_start,
            call_method_or_get_field!(
                env,
                raycast_context,
                "getStart",
                false,
                &[]
            ).unwrap().l().unwrap()
        );
        let raycast_end = mappings_manager.get("Vec3d").unwrap();
        apply_object!(
            raycast_end,
            call_method_or_get_field!(
                env,
                raycast_context,
                "getEnd",
                false,
                &[]
            ).unwrap().l().unwrap()
        );

        let voxel_shape = mappings_manager.get("VoxelShape").unwrap();
        apply_object!(
            voxel_shape,
            call_method_or_get_field!(
                env,
                raycast_context,
                "getBlockShape",
                false,
                &[
                    JValue::Object(block_state.get_object().unwrap()),
                    JValue::Object(world.get_object().unwrap()),
                    JValue::Object(block_pos.get_object().unwrap())
                ]
            ).unwrap().l().unwrap()
        );

        let block_hit_result = mappings_manager.get("BlockHitResult").unwrap();
        apply_object!(
            block_hit_result,
            call_method_or_get_field!(
                env,
                world,
                "raycastBlock",
                false,
                &[
                    JValue::Object(raycast_start.get_object().unwrap()),
                    JValue::Object(raycast_end.get_object().unwrap()),
                    JValue::Object(block_pos.get_object().unwrap()),
                    JValue::Object(voxel_shape.get_object().unwrap()),
                    JValue::Object(block_state.get_object().unwrap())
                ]
            ).unwrap().l().unwrap()
        );

        let other_voxel_shape = mappings_manager.get("VoxelShape").unwrap();
        apply_object!(
            other_voxel_shape,
            call_method_or_get_field!(
                env,
                mappings_manager.get("VoxelShapes").unwrap(),
                "empty",
                true,
                &[]
            ).unwrap().l().unwrap()
        );

        let other_block_hit_result = mappings_manager.get("BlockHitResult").unwrap();
        apply_object!(
            other_block_hit_result,
            call_method_or_get_field!(
                env,
                other_voxel_shape,
                "raycast",
                false,
                &[
                    JValue::Object(raycast_start.get_object().unwrap()),
                    JValue::Object(raycast_end.get_object().unwrap()),
                    JValue::Object(block_pos.get_object().unwrap())
                ]
            ).unwrap().l().unwrap()
        );

        let d = if env.is_same_object(block_hit_result.get_object().unwrap(), JObject::null()).unwrap() {
            f64::MAX
        } else {
            let get_coords_from = mappings_manager.get("BlockPos").unwrap();
            apply_object!(
                get_coords_from,
                call_method_or_get_field!(
                    env,
                    block_hit_result,
                    "getPos",
                    false,
                    &[]
                ).unwrap().l().unwrap()
            );
            let x = call_method_or_get_field!(
                env,
                get_coords_from,
                "getX",
                false,
                &[]
            ).unwrap().i().unwrap() as f64;
            let y = call_method_or_get_field!(
                env,
                get_coords_from,
                "getY",
                false,
                &[]
            ).unwrap().i().unwrap() as f64;
            let z = call_method_or_get_field!(
                env,
                get_coords_from,
                "getZ",
                false,
                &[]
            ).unwrap().i().unwrap() as f64;

            call_method_or_get_field!(
                env,
                raycast_start,
                "distanceToSqr",
                false,
                &[
                    JValue::Double(x),
                    JValue::Double(y),
                    JValue::Double(z)
                ]
            ).unwrap().d().unwrap()
        };
        let e = if env.is_same_object(other_block_hit_result.get_object().unwrap(), JObject::null()).unwrap() {
            f64::MAX
        } else {
            let get_coords_from = mappings_manager.get("BlockPos").unwrap();
            apply_object!(
                get_coords_from,
                call_method_or_get_field!(
                    env,
                    other_block_hit_result,
                    "getPos",
                    false,
                    &[]
                ).unwrap().l().unwrap()
            );
            let x = call_method_or_get_field!(
                env,
                get_coords_from,
                "getX",
                false,
                &[]
            ).unwrap().i().unwrap() as f64;
            let y = call_method_or_get_field!(
                env,
                get_coords_from,
                "getY",
                false,
                &[]
            ).unwrap().i().unwrap() as f64;
            let z = call_method_or_get_field!(
                env,
                get_coords_from,
                "getZ",
                false,
                &[]
            ).unwrap().i().unwrap() as f64;

            call_method_or_get_field!(
                env,
                raycast_start,
                "distanceToSqr",
                false,
                &[
                    JValue::Double(x),
                    JValue::Double(y),
                    JValue::Double(z)
                ]
            ).unwrap().d().unwrap()
        };

        if d <= e {
            block_hit_result
        } else {
            other_block_hit_result
        }
    };
    let miss_closure = || {
        let make_missed_with = mappings_manager.get("Vec3d").unwrap();
        let end_x = call_method_or_get_field!(
            env,
            end_vec3d,
            "getX",
            false,
            &[]
        ).unwrap().d().unwrap();
        let end_y = call_method_or_get_field!(
            env,
            end_vec3d,
            "getY",
            false,
            &[]
        ).unwrap().d().unwrap();
        let end_z = call_method_or_get_field!(
            env,
            end_vec3d,
            "getZ",
            false,
            &[]
        ).unwrap().d().unwrap();
        apply_object!(
            make_missed_with,
            call_method_or_get_field!(
                env,
                start_vec3d,
                "subtract",
                false,
                &[
                    JValue::Double(end_x),
                    JValue::Double(end_y),
                    JValue::Double(end_z)
                ]
            ).unwrap().l().unwrap()
        );

        let missed_block_hit_result = mappings_manager.get("BlockHitResult").unwrap();
        let direction = mappings_manager.get("Direction").unwrap();
        apply_object!(
            direction,
            call_method_or_get_field!(
                env,
                direction,
                "getFacing",
                true,
                &[
                    JValue::Double(end_x),
                    JValue::Double(end_y),
                    JValue::Double(end_z)
                ]
            ).unwrap().l().unwrap()
        );
        let missed_block_pos = mappings_manager.get("BlockPos").unwrap();
        apply_object!(
            missed_block_pos,
            call_method_or_get_field!(
                env,
                missed_block_pos,
                "<init>",
                true,
                &[
                    JValue::Object(end_vec3d.get_object().unwrap()),  // this might break
                ]
            ).unwrap().l().unwrap()
        );
        apply_object!(
            missed_block_hit_result,
            call_method_or_get_field!(
                env,
                missed_block_hit_result,
                "createMissed",
                false,
                &[
                    JValue::Object(end_vec3d.get_object().unwrap()),
                    JValue::Object(direction.get_object().unwrap()),
                    JValue::Object(missed_block_pos.get_object().unwrap()),
                ]
            ).unwrap().l().unwrap()
        );
        missed_block_hit_result
    };

    if env.is_same_object(start_vec3d.get_object().unwrap(), end_vec3d.get_object().unwrap()).unwrap() {
        // miss closure
    } else {
        // hit
        let math_helper = mappings_manager.get("MathHelper").unwrap();
        // holy bananas
        let d = call_method_or_get_field!(
            env,
            math_helper,
            "lerp",
            true,
            &[
                JValue::Double(-1.0E-7),
                JValue::Double(call_method_or_get_field!(
                    env,
                    end_vec3d,
                    "x",
                    false
                ).unwrap().d().unwrap()),
                JValue::Double(call_method_or_get_field!(
                    env,
                    start_vec3d,
                    "x",
                    false
                ).unwrap().d().unwrap())
            ]
        ).unwrap().d().unwrap();
        let e = call_method_or_get_field!(
            env,
            math_helper,
            "lerp",
            true,
            &[
                JValue::Double(-1.0E-7),
                JValue::Double(call_method_or_get_field!(
                    env,
                    end_vec3d,
                    "y",
                    false
                ).unwrap().d().unwrap()),
                JValue::Double(call_method_or_get_field!(
                    env,
                    start_vec3d,
                    "y",
                    false
                ).unwrap().d().unwrap())
            ]
        ).unwrap().d().unwrap();
        let f = call_method_or_get_field!(
            env,
            math_helper,
            "lerp",
            true,
            &[
                JValue::Double(-1.0E-7),
                JValue::Double(call_method_or_get_field!(
                    env,
                    end_vec3d,
                    "z",
                    false
                ).unwrap().d().unwrap()),
                JValue::Double(call_method_or_get_field!(
                    env,
                    start_vec3d,
                    "z",
                    false
                ).unwrap().d().unwrap())
            ]
        ).unwrap().d().unwrap();
        let g = call_method_or_get_field!(
            env,
            math_helper,
            "lerp",
            true,
            &[
                JValue::Double(-1.0E-7),
                JValue::Double(call_method_or_get_field!(
                    env,
                    start_vec3d,
                    "x",
                    false
                ).unwrap().d().unwrap()),
                JValue::Double(call_method_or_get_field!(
                    env,
                    end_vec3d,
                    "x",
                    false
                ).unwrap().d().unwrap())
            ]
        ).unwrap().d().unwrap();
        let h = call_method_or_get_field!(
            env,
            math_helper,
            "lerp",
            true,
            &[
                JValue::Double(-1.0E-7),
                JValue::Double(call_method_or_get_field!(
                    env,
                    start_vec3d,
                    "y",
                    false
                ).unwrap().d().unwrap()),
                JValue::Double(call_method_or_get_field!(
                    env,
                    end_vec3d,
                    "y",
                    false
                ).unwrap().d().unwrap())
            ]
        ).unwrap().d().unwrap();
        let i = call_method_or_get_field!(
            env,
            math_helper,
            "lerp",
            true,
            &[
                JValue::Double(-1.0E-7),
                JValue::Double(call_method_or_get_field!(
                    env,
                    start_vec3d,
                    "z",
                    false
                ).unwrap().d().unwrap()),
                JValue::Double(call_method_or_get_field!(
                    env,
                    end_vec3d,
                    "z",
                    false
                ).unwrap().d().unwrap())
            ]
        ).unwrap().d().unwrap();
        let j = call_method_or_get_field!(
            env,
            math_helper,
            "floor",
            true,
            &[
                JValue::Double(g)
            ]
        ).unwrap().i().unwrap();
        let k = call_method_or_get_field!(
            env,
            math_helper,
            "floor",
            true,
            &[
                JValue::Double(h)
            ]
        ).unwrap().i().unwrap();
        let l = call_method_or_get_field!(
            env,
            math_helper,
            "floor",
            true,
            &[
                JValue::Double(i)
            ]
        ).unwrap().i().unwrap();

        let maybe_null_object = mappings_manager.get("BlockHitResult").unwrap();
        //apply_object!(
        //    maybe_null_object,
        //    hit_closure(
        //        raycast_context,
        //        bl
        //    )
        //)
    }
    todo!()
}
