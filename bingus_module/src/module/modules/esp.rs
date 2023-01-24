use crate::crate_prelude::*;


fn tick(esp: &mut Esp, env: JNIEnv, mappings_manager: &MappingsManager) {
    let minecraft_client = get_minecraft_client(env, mappings_manager);
    let player = match get_player_checked(env, mappings_manager, minecraft_client) {
        Some(player) => player,
        None => return,
    };
    let world = match get_world_checked(env, mappings_manager, minecraft_client) {
        Some(world) => world,
        None => return,
    };

    let entities_iterator = mappings_manager.get("Iterator").unwrap();
    apply_object!(
        entities_iterator,
        java_iterable_to_iterator(
            env,
            mappings_manager,
            {
                let iterable = mappings_manager.get("Iterable").unwrap();
                apply_object!(
                    iterable,
                    call_method_or_get_field!(
                        env,
                        world,
                        "getEntities",
                        false,
                        &[]
                    ).unwrap().l().unwrap()
                );
                iterable
            }
        ).get_object().unwrap()
    );

    esp.entity_list = vec![];
    loop {
        if let Some(entity) = get_next_java_iterator_checked(env, mappings_manager, entities_iterator) {  // entity is a generic Object ClassMapping, which is casted to an Entity ClassMapping
            let entity = {
                let _entity = entity;
                let entity = mappings_manager.get("Entity").unwrap();
                apply_object!(entity, _entity.get_object().unwrap());
                entity
            };

            if {
                env.is_same_object(entity.get_object().unwrap(), JObject::null()).unwrap() ||
                !is_instance_of(env, entity, mappings_manager.get("LivingEntity").unwrap()) ||
                get_entity_id(env, entity) == get_entity_id(env, player)
            } {
                continue;
            }

            esp.entity_list.push(RenderInfo::new(
                get_entity_pos_array(env, mappings_manager, entity),
                get_entity_bounding_box_minmax_array(env, mappings_manager, entity),
            ));
        } else {
            break;
        }
    }
}

fn render(esp: &mut Esp, env: JNIEnv, mappings_manager: &MappingsManager) {
    let minecraft_client = get_minecraft_client(env, mappings_manager);
    let player = match get_player_checked(env, mappings_manager, minecraft_client) {
        Some(player) => player,
        None => return,
    };
    let render_tick_manager = mappings_manager.get("RenderTickCounter").unwrap();  // timer
    apply_object!(
        render_tick_manager,
        call_method_or_get_field!(
            env,
            minecraft_client,
            "renderTickCounter",
            false
        ).unwrap().l().unwrap()
    );

    let viewport = get_viewport(env, minecraft_client);
    let modelview_matrix = get_matrix_16(env, get_modelview_class_mapping(env, mappings_manager));
    let projection_matrix = get_matrix_16(env, get_projection_class_mapping(env, mappings_manager));

    let [render_x, render_y, render_z] = get_entity_pos_array(env, mappings_manager, player);
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
    };

    for entity in esp.entity_list {
        
    }
}

#[derive(BingusModuleTrait)]
#[add_bingus_fields]
#[bingus_module(name = "ESP (doesn't work)", tick_method = "tick(self, _env, _mappings_manager)", render_method = "render(self, _env, _mappings_manager)")]
pub struct Esp {
    entity_list: Vec<RenderInfo>,
}

impl MakeNewBingusModule for Esp {
    fn new() -> Self {
        Self {
            entity_list: vec![],
            __enabled_bool_setting: (BingusSetting::BoolSetting(false.into()), "enabled", None),
            __keybind_setting: (BingusSetting::KeySetting(String::from("").into()), "keybind", None),
            __env: None,
            __mappings_manager: None,
        }
    }
}