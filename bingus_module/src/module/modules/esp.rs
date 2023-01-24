use glam::{Vec4, Vec2, Vec3};
use glu_sys::{glRectf, glClearColor, glClear, GL_COLOR_BUFFER_BIT};

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
                get_entity_bounding_box_minmax_array(env, mappings_manager, minecraft_client, player, entity),
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

    let viewport = get_viewport(env, mappings_manager);
    let modelview_matrix = get_matrix_16(env, get_modelview_class_mapping(env, mappings_manager));
    let projection_matrix = get_matrix_16(env, get_projection_class_mapping(env, mappings_manager));


    for entity in &esp.entity_list {
        let bounding_box = entity.get_bounding_box();  // minx miny minz maxx maxy maxz
        let box_vertices: [[f64; 3]; 8] = [
            [bounding_box[0] - 0.1, bounding_box[1] - 0.1, bounding_box[2] - 0.1],
            [bounding_box[0] - 0.1, bounding_box[4] + 0.1, bounding_box[2] - 0.1],
            [bounding_box[3] + 0.1, bounding_box[4] + 0.1, bounding_box[2] - 0.1],
            [bounding_box[3] + 0.1, bounding_box[1] - 0.1, bounding_box[2] - 0.1],
            [bounding_box[3] + 0.1, bounding_box[4] + 0.1, bounding_box[5] + 0.1],
            [bounding_box[0] - 0.1, bounding_box[4] + 0.1, bounding_box[5] + 0.1],
            [bounding_box[0] - 0.1, bounding_box[1] - 0.1, bounding_box[5] + 0.1],
            [bounding_box[3] + 0.1, bounding_box[1] - 0.1, bounding_box[5] + 0.1]
        ];

        let mut w2sbb = Vec4::new(f32::MAX, f32::MAX, -1.0, -1.0);

        for i in 0..8 {
            if let Some(screen_pos) = world_to_screen(
                Vec3::new(box_vertices[i][0] as f32, box_vertices[i][1] as f32, box_vertices[i][2] as f32),
                modelview_matrix,
                projection_matrix,
                viewport
            ) {
                w2sbb.x = if w2sbb.x < screen_pos.x {
                    w2sbb.x
                } else {
                    screen_pos.x
                };
                w2sbb.y = if w2sbb.x < screen_pos.y {
                    w2sbb.x
                } else {
                    screen_pos.y
                };
                w2sbb.z = if w2sbb.z > screen_pos.x {
                    w2sbb.z
                } else {
                    screen_pos.x
                };
                w2sbb.w = if w2sbb.w > screen_pos.y {
                    w2sbb.w
                } else {
                    screen_pos.y
                };
            }
        }


        if w2sbb.x >= 0.0 || w2sbb.y >= 0.0 || w2sbb.z <= viewport[2] || w2sbb.w <= viewport[3] {
            unsafe {
                glClearColor(0.1, 0.8, 0.1, 1.0);
                glClear(GL_COLOR_BUFFER_BIT);
                glRectf(w2sbb.x, w2sbb.y, w2sbb.z, w2sbb.w)
            }
        }
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