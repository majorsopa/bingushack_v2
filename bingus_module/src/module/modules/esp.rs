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


    let mut entity_list = vec![];
    loop {
        if let Some(entity) = get_next_java_iterator_checked(env, mappings_manager, entities_iterator) {  // entity is a generic Object ClassMapping, which is casted to an Entity ClassMapping
            let entity = {
                let _entity = entity;
                let entity = mappings_manager.get("Entity").unwrap();
                apply_object!(entity, _entity.get_object().unwrap());
                entity
            };

            if env.is_same_object(entity.get_object().unwrap(), JObject::null()).unwrap() ||
            !is_instance_of(env, entity, mappings_manager.get("LivingEntity").unwrap()) ||
            get_entity_id(env, entity) == get_entity_id(env, {
                let entity = mappings_manager.get("Entity").unwrap();
                apply_object!(entity, player.get_object().unwrap());
                entity
            }) {
                continue;
            }

            entity_list.push(RenderInfo::new_from_entity(env, mappings_manager, entity));
        } else {
            break;
        }
    }


    // IMPORTANT
    if !esp.rects.is_empty() {
        return;
    }
    for entity in entity_list {
        esp.rects.push({
            let [x, y] = world_to_screen(entity.entity_pos);

            [x, y, x + 1.0, y + 1.0]
        });
    }
    //send_chat_message(env, mappings_manager, player, &format!("{rects:#?}"));
    render_outline(
        env,
        mappings_manager,
        minecraft_client,
        [0, 0],
        20,
        20,
        200
    );
}

fn render(esp: &mut Esp, env: JNIEnv, mappings_manager: &MappingsManager) {
    
}


#[derive(BingusModuleTrait)]
#[add_bingus_fields]
#[bingus_module(
    name = "ESP (doesn't work)",
    tick_method = "tick(self, _env, _mappings_manager)",
    render_method = "render(self, _env, _mappings_manager)",
)]
pub struct Esp {
    rects: Vec<[f32; 4]>,
}

impl MakeNewBingusModule for Esp {
    fn new() -> Self {
        Self {
            rects: vec![],
            __enabled_bool_setting: (BingusSetting::BoolSetting(false.into()), "enabled", None),
            __keybind_setting: (BingusSetting::KeySetting(String::from("").into()), "keybind", None),
            __env: None,
            __mappings_manager: None,
            __prev_enabled: false,
        }
    }
}
