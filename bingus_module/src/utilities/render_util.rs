use crate::crate_prelude::*;



pub struct RenderInfo {
    pub entity_pos: [f64; 3]
}

impl RenderInfo {
    pub fn new_from_entity<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, entity: &'a ClassMapping<'a>) -> Self {
        Self {
            entity_pos: get_entity_pos_array(env, mappings_manager, entity)
        }
    }
}


// the casting might be a bruh moment
pub fn get_viewport<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager) -> [f32; 4] {
    let viewport_class_mapping = mappings_manager.get("Viewport").unwrap();
    let x = call_method_or_get_field!(
        env,
        viewport_class_mapping,
        "x",
        true
    ).unwrap().i().unwrap() as f32;
    let y = call_method_or_get_field!(
        env,
        viewport_class_mapping,
        "y",
        true
    ).unwrap().i().unwrap() as f32;
    let width = call_method_or_get_field!(
        env,
        viewport_class_mapping,
        "width",
        true
    ).unwrap().i().unwrap() as f32;
    let height = call_method_or_get_field!(
        env,
        viewport_class_mapping,
        "height",
        true
    ).unwrap().i().unwrap() as f32;

    [x, y, width, height]
}

pub fn get_matrix_16<'a>(env: JNIEnv<'a>, matrix_class_mapping: &'a ClassMapping<'a>) -> [f32; 16] {
    let mut matrix = [0.0; 16];

    for i in 0..4 {
        for j in 0..4 {
            let field_name = format!("m{}{}", i, j);
            let field = call_method_or_get_field!(
                env,
                matrix_class_mapping,
                field_name.as_str(),
                true
            ).unwrap().f().unwrap();

            matrix[i * 4 + j] = field;
        }
    }

    matrix
}

pub fn world_to_screen<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, player: &'a ClassMapping, entity_pos: [f64; 3]) -> [f64; 2] {
    let camera_position = get_player_pos(env, mappings_manager, player);
    let mut camera_position = [entity_pos[0] - camera_position[0], entity_pos[1] - camera_position[1], entity_pos[2] - camera_position[2]];
    let [pitch, yaw] = get_pitch_and_yaw(
        env,
        get_camera(
            env,
            mappings_manager,
            get_game_renderer(
                env,
                mappings_manager,
                get_minecraft_client(
                    env,
                    mappings_manager
                )
            )
        )
    ).map(|c| c as f64);
    // x and z might need to be swapped
    camera_position[2] *= yaw.cos();
    camera_position[0] *= yaw.sin();
    camera_position[2] *= pitch.cos();

    camera_position[1] *= pitch.sin();

    camera_position[0] /= camera_position[2] * 2.0;
    camera_position[1] /= camera_position[2] * 2.0;

    [camera_position[0], camera_position[1]]
}

pub fn get_game_renderer<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, minecraft_client: &'a ClassMapping) -> &'a ClassMapping<'a> {
    let game_renderer = mappings_manager.get("GameRenderer").unwrap();
    apply_object!(
        game_renderer,
        call_method_or_get_field!(env, minecraft_client, "gameRenderer", false).unwrap().l().unwrap()
    );
    game_renderer
}

pub fn get_camera<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, game_renderer: &'a ClassMapping) -> &'a ClassMapping<'a> {
    let camera = mappings_manager.get("Camera").unwrap();
    apply_object!(
        camera,
        call_method_or_get_field!(env, game_renderer, "getCamera", false, &[]).unwrap().l().unwrap()
    );
    camera
}

pub fn get_pitch_and_yaw<'a>(env: JNIEnv<'a>, camera: &'a ClassMapping) -> [f32; 2] {
    let pitch = call_method_or_get_field!(
        env,
        camera,
        "pitch",
        false
    ).unwrap().f().unwrap();
    let yaw = call_method_or_get_field!(
        env,
        camera,
        "yaw",
        false
    ).unwrap().f().unwrap();

    [pitch, yaw]
}
