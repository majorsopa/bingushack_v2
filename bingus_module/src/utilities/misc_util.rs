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

pub fn math_helper_lerp<'a>(env: JNIEnv<'a>, math_helper: &'a ClassMapping<'a>, a: f64, b: f64, c: f64) -> f64 {
    call_method_or_get_field!(
        env,
        math_helper,
        "lerp",
        true,
        &[
            JValue::Double(a),
            JValue::Double(b),
            JValue::Double(c)
        ]
    ).unwrap().d().unwrap()
}

pub fn math_helper_floor<'a>(env: JNIEnv<'a>, math_helper: &'a ClassMapping<'a>, a: f64) -> i32 {
    call_method_or_get_field!(
        env,
        math_helper,
        "floor",
        true,
        &[
            JValue::Double(a)
        ]
    ).unwrap().i().unwrap()
}

pub fn math_helper_sign<'a>(env: JNIEnv<'a>, math_helper: &'a ClassMapping<'a>, a: f64) -> i32 {
    call_method_or_get_field!(
        env,
        math_helper,
        "sign",
        true,
        &[
            JValue::Double(a)
        ]
    ).unwrap().i().unwrap()
}

pub fn raycast_replacement<'a>(
    env: JNIEnv<'a>,
    mappings_manager: &'a MappingsManager,
    world: &'a ClassMapping<'a>,
    raycast_context: &'a ClassMapping<'a>,
    obby_pos: &'a ClassMapping<'a>,
    ignore_terrain: bool
) -> &'a ClassMapping<'a> {
    let start_vec3d = mappings_manager.get("Vec3d").unwrap();
    apply_object!(
        start_vec3d,
        call_method_or_get_field!(
            env,
            raycast_context,
            "getStart",
            false,
            &[]
        ).unwrap().l().unwrap()
    );
    let end_vec3d = mappings_manager.get("Vec3d").unwrap();
    apply_object!(
        end_vec3d,
        call_method_or_get_field!(
            env,
            raycast_context,
            "getEnd",
            false,
            &[]
        ).unwrap().l().unwrap()
    );

    let hit_closure = |block_pos: &'a ClassMapping<'a>| {
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
                    get_minecraft_client(env, mappings_manager),
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
                    JValue::Object(start_vec3d.get_object().unwrap()),
                    JValue::Object(end_vec3d.get_object().unwrap()),
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
                    JValue::Object(start_vec3d.get_object().unwrap()),
                    JValue::Object(end_vec3d.get_object().unwrap()),
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
                start_vec3d,
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
                start_vec3d,
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
        miss_closure()
    } else {
        // hit
        let math_helper = mappings_manager.get("MathHelper").unwrap();
        // holy bananas
        let d = math_helper_lerp(
            env,
            math_helper,
            -1.0E-7,
            call_method_or_get_field!(
                env,
                end_vec3d,
                "x",
                false
            ).unwrap().d().unwrap(),
            call_method_or_get_field!(
                env,
                start_vec3d,
                "x",
                false
            ).unwrap().d().unwrap()
        );
        let e = math_helper_lerp(
            env,
            math_helper,
            -1.0E-7,
            call_method_or_get_field!(
                env,
                end_vec3d,
                "y",
                false
            ).unwrap().d().unwrap(),
            call_method_or_get_field!(
                env,
                start_vec3d,
                "y",
                false
            ).unwrap().d().unwrap()
        );
        let f = math_helper_lerp(
            env,
            math_helper,
            -1.0E-7,
            call_method_or_get_field!(
                env,
                end_vec3d,
                "z",
                false
            ).unwrap().d().unwrap(),
            call_method_or_get_field!(
                env,
                start_vec3d,
                "z",
                false
            ).unwrap().d().unwrap()
        );
        let g = math_helper_lerp(
            env,
            math_helper,
            -1.0E-7,
            call_method_or_get_field!(
                env,
                start_vec3d,
                "x",
                false
            ).unwrap().d().unwrap(),
            call_method_or_get_field!(
                env,
                end_vec3d,
                "x",
                false
            ).unwrap().d().unwrap(),
        );
        let h = math_helper_lerp(
            env,
            math_helper,
            -1.0E-7,
            call_method_or_get_field!(
                env,
                start_vec3d,
                "y",
                false
            ).unwrap().d().unwrap(),
            call_method_or_get_field!(
                env,
                end_vec3d,
                "y",
                false
            ).unwrap().d().unwrap()
        );
        let i = math_helper_lerp(
            env,
            math_helper,
            -1.0E-7,
            call_method_or_get_field!(
                env,
                start_vec3d,
                "z",
                false
            ).unwrap().d().unwrap(),
            call_method_or_get_field!(
                env,
                end_vec3d,
                "z",
                false
            ).unwrap().d().unwrap()
        );
        let mut j = math_helper_floor(env, math_helper, g);
        let mut k = math_helper_floor(env, math_helper, h);
        let mut l = math_helper_floor(env, math_helper, i);

        let mutable_block_pos = mappings_manager.get("MutableBlockPos").unwrap();
        apply_object!(
            mutable_block_pos,
            call_method_or_get_field!(
                env,
                mutable_block_pos,
                "<init>",
                true,
                &[
                    JValue::Int(j),
                    JValue::Int(k),
                    JValue::Int(l)
                ]
            ).unwrap().l().unwrap()
        );


        let maybe_null_block_hit_result = mappings_manager.get("BlockHitResult").unwrap();
        apply_object!(
            maybe_null_block_hit_result,
            hit_closure(mutable_block_pos).get_object().unwrap()
        );

        if !env.is_same_object(maybe_null_block_hit_result.get_object().unwrap(), JObject::null()).unwrap() {
            return maybe_null_block_hit_result;
        } else {
            let m = d - g;
            let n = e - h;
            let o = f - i;

            let p = call_method_or_get_field!(
                env,
                math_helper,
                "sign",
                true,
                &[
                    JValue::Double(m)
                ]
            ).unwrap().i().unwrap();
            let q = call_method_or_get_field!(
                env,
                math_helper,
                "sign",
                true,
                &[
                    JValue::Double(n)
                ]
            ).unwrap().i().unwrap();
            let r = call_method_or_get_field!(
                env,
                math_helper,
                "sign",
                true,
                &[
                    JValue::Double(o)
                ]
            ).unwrap().i().unwrap();

            let s = if p == 0 {
                f64::MAX
            } else {
                p as f64 / m
            };
            let t = if q == 0 {
                f64::MAX
            } else {
                q as f64 / n
            };
            let u = if r == 0 {
                f64::MAX
            } else {
                r as f64 / o
            };
            let mut v = s * (if p > 0 {
                1.0
            } else {
                0.0
            } - call_method_or_get_field!(
                env,
                math_helper,
                "fractionalPart",
                true,
                &[
                    JValue::Double(g)
                ]
            ).unwrap().d().unwrap());
            let mut w = t * (if q > 0 {
                1.0
            } else {
                0.0
            } - call_method_or_get_field!(
                env,
                math_helper,
                "fractionalPart",
                true,
                &[
                    JValue::Double(h)
                ]
            ).unwrap().d().unwrap());
            let mut x = u * (if r > 0 {
                1.0
            } else {
                0.0
            } - call_method_or_get_field!(
                env,
                math_helper,
                "fractionalPart",
                true,
                &[
                    JValue::Double(i)
                ]
            ).unwrap().d().unwrap());

            let block_hit_result2 = mappings_manager.get("BlockHitResult").unwrap();
            while {
                if {
                    !(v <= 1.0)
                    && !(w <= 1.0)
                    && !(x <= 1.0)
                } {
                    return miss_closure();
                }

                // literally copy-pasted from the Java code LOL
                if v < w {
                    if v < x {
                        j += p;
                        v += s;
                    } else {
                        l += r;
                        x += u;
                    }
                } else if w < x {
                    k += q;
                    w += t;
                } else {
                    l += r;
                    x += u;
                }



                apply_object!(
                    mutable_block_pos,
                    call_method_or_get_field!(
                        env,
                        mutable_block_pos,
                        "set",
                        false,
                        &[
                            JValue::Int(j),
                            JValue::Int(k),
                            JValue::Int(l)
                        ]
                    ).unwrap().l().unwrap()
                );
                apply_object!(
                    block_hit_result2,
                    hit_closure(mutable_block_pos).get_object().unwrap()
                );

                env.is_same_object(block_hit_result2.get_object().unwrap(), JObject::null()).unwrap()
            } {}  // rust do-while loop be like

            block_hit_result2
        }
    }
}

pub fn get_exposure<'a>(
    env: JNIEnv<'a>,
    mappings_manager: &'a MappingsManager,
    source_vec3d: &'a ClassMapping<'a>,
    entity: &'a ClassMapping<'a>,
    player_pos: &'a ClassMapping<'a>,
    obby_pos: &'a ClassMapping<'a>,
    ignore_terrain: bool,
) -> f64 {
    let world = match get_world_checked(env, mappings_manager, get_minecraft_client(env, mappings_manager)) {
        Some(world) => world,
        None => return 0.0,
    };

    let box_class = mappings_manager.get("Box").unwrap();
    apply_object!(
        box_class,
        call_method_or_get_field!(
            env,
            entity,
            "calculateBoundingBox",
            false,
            &[]
        ).unwrap().l().unwrap()
    );
    let v = mappings_manager.get("Vec3d").unwrap();
    let [x, y, z] = [
        call_method_or_get_field!(
            env,
            player_pos,
            "x",
            false
        ).unwrap().d().unwrap(),
        call_method_or_get_field!(
            env,
            player_pos,
            "y",
            false
        ).unwrap().d().unwrap(),
        call_method_or_get_field!(
            env,
            player_pos,
            "z",
            false
        ).unwrap().d().unwrap()
    ];
    apply_object!(
        v,
        call_method_or_get_field!(
            env,
            player_pos,
            "subtract",
            false,
            &[
                JValue::Double(x),
                JValue::Double(y),
                JValue::Double(z)
            ]
        ).unwrap().l().unwrap()
    );
    apply_object!(
        box_class,
        call_method_or_get_field!(
            env,
            box_class,
            "offset",
            false,
            &[JValue::Object(v.get_object().unwrap())]
        ).unwrap().l().unwrap()
    );

    let d = 1.0 / (call_method_or_get_field!(
        env,
        box_class,
        "maxX",
        false
    ).unwrap().d().unwrap() - call_method_or_get_field!(
        env,
        box_class,
        "minX",
        false
    ).unwrap().d().unwrap() * 2.0 + 1.0);
    let e = 1.0 / (call_method_or_get_field!(
        env,
        box_class,
        "maxY",
        false
    ).unwrap().d().unwrap() - call_method_or_get_field!(
        env,
        box_class,
        "minY",
        false
    ).unwrap().d().unwrap() * 2.0 + 1.0);
    let f = 1.0 / (call_method_or_get_field!(
        env,
        box_class,
        "maxZ",
        false
    ).unwrap().d().unwrap() - call_method_or_get_field!(
        env,
        box_class,
        "minZ",
        false
    ).unwrap().d().unwrap() * 2.0 + 1.0);
    let g = (1.0 - call_method_or_get_field!(
        env,
        mappings_manager.get("MathHelper").unwrap(),
        "floor",
        true,
        &[
            JValue::Double(1.0 / d)
        ]
    ).unwrap().d().unwrap() * d) / 2.0;
    let h = (1.0 - call_method_or_get_field!(
        env,
        mappings_manager.get("MathHelper").unwrap(),
        "floor",
        true,
        &[
            JValue::Double(1.0 / f)
        ]
    ).unwrap().d().unwrap() * f) / 2.0;

    if {
        !(d < 0.0)
        && !(e < 0.0)
        && !(f < 0.0)
    } {
        let mut i = 0;
        let mut j = 0;

        let math_helper = mappings_manager.get("MathHelper").unwrap();

        let mut k = 0.0;
        while k <= 1.0 {
            let mut l = 0.0;
            while l <= 1.0 {
                let mut m = 0.0;
                while m <= 1.0 {
                    let n = call_method_or_get_field!(
                        env,
                        math_helper,
                        "lerp",
                        true,
                        &[
                            JValue::Double(k),
                            JValue::Double(call_method_or_get_field!(
                                env,
                                box_class,
                                "minX",
                                false
                            ).unwrap().d().unwrap()),
                            JValue::Double(call_method_or_get_field!(
                                env,
                                box_class,
                                "maxX",
                                false
                            ).unwrap().d().unwrap())
                        ]
                    ).unwrap().d().unwrap();
                    let o = call_method_or_get_field!(
                        env,
                        math_helper,
                        "lerp",
                        true,
                        &[
                            JValue::Double(l),
                            JValue::Double(call_method_or_get_field!(
                                env,
                                box_class,
                                "minY",
                                false
                            ).unwrap().d().unwrap()),
                            JValue::Double(call_method_or_get_field!(
                                env,
                                box_class,
                                "maxY",
                                false
                            ).unwrap().d().unwrap())
                        ]
                    ).unwrap().d().unwrap();
                    let p = call_method_or_get_field!(
                        env,
                        math_helper,
                        "lerp",
                        true,
                        &[
                            JValue::Double(m),
                            JValue::Double(call_method_or_get_field!(
                                env,
                                box_class,
                                "minZ",
                                false
                            ).unwrap().d().unwrap()),
                            JValue::Double(call_method_or_get_field!(
                                env,
                                box_class,
                                "maxZ",
                                false
                            ).unwrap().d().unwrap())
                        ]
                    ).unwrap().d().unwrap();

                    let vec3d = mappings_manager.get("Vec3d").unwrap();
                    apply_object!(
                        vec3d,
                        call_method_or_get_field!(
                            env,
                            vec3d,
                            "<init>",
                            true,
                            &[
                                JValue::Double(n + g),
                                JValue::Double(o),
                                JValue::Double(p + h)
                            ]
                        ).unwrap().l().unwrap()
                    );
                    let collider = mappings_manager.get("ShapeType").unwrap();
                    apply_object!(
                        collider,
                        call_method_or_get_field!(
                            env,
                            collider,
                            "COLLIDER",
                            true
                        ).unwrap().l().unwrap()
                    );
                    let fluid_handling = mappings_manager.get("FluidHandling").unwrap();
                    apply_object!(
                        fluid_handling,
                        call_method_or_get_field!(
                            env,
                            fluid_handling,
                            "NONE",
                            true
                        ).unwrap().l().unwrap()
                    );
                    let raycast_context = new_raycast_context(env, mappings_manager, vec3d, source_vec3d, collider, fluid_handling, entity);
                    if env.is_same_object(
                        raycast_replacement(env, mappings_manager, world, raycast_context, obby_pos, ignore_terrain).get_object().unwrap(),
                        {
                            let hit_result_type = mappings_manager.get("HitResultType").unwrap();
                            apply_object!(
                                hit_result_type,
                                call_method_or_get_field!(
                                    env,
                                    hit_result_type,
                                    "MISS",
                                    true
                                ).unwrap().l().unwrap()
                            );
                            hit_result_type
                        }.get_object().unwrap()
                    ).unwrap() {
                        i += 1;
                    }
                    j += 1;


                    m += f;
                }

                l += e;
            }

            k += d;
        }

        i as f64 / j as f64
    } else {
        0.0
    }
}

pub fn new_raycast_context<'a>(
    env: JNIEnv<'a>,
    mappings_manager: &'a MappingsManager,
    vec3d: &'a ClassMapping<'a>,
    source_vec3d: &'a ClassMapping<'a>,
    shape_type: &'a ClassMapping<'a>,
    fluid_handling: &'a ClassMapping<'a>,
    entity: &'a ClassMapping<'a>,
) -> &'a ClassMapping<'a> {
    let raycast_context = mappings_manager.get("RaycastContext").unwrap();
    apply_object!(
        raycast_context,
        call_method_or_get_field!(
            env,
            raycast_context,
            "<init>",
            true,
            &[
                JValue::Object(vec3d.get_object().unwrap()),
                JValue::Object(source_vec3d.get_object().unwrap()),
                JValue::Object(shape_type.get_object().unwrap()),
                JValue::Object(fluid_handling.get_object().unwrap()),
                JValue::Object(entity.get_object().unwrap())
            ]
        ).unwrap().l().unwrap()
    );
    raycast_context
}
