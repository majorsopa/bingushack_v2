use nalgebra_glm::{Vec3, Vec4, Mat4, perspective, look_at};

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
        "getX",
        true,
        &[]
    ).unwrap().i().unwrap() as f32;
    let y = call_method_or_get_field!(
        env,
        viewport_class_mapping,
        "getY",
        true,
        &[]
    ).unwrap().i().unwrap() as f32;
    let width = call_method_or_get_field!(
        env,
        viewport_class_mapping,
        "getWidth",
        true,
        &[]
    ).unwrap().i().unwrap() as f32;
    let height = call_method_or_get_field!(
        env,
        viewport_class_mapping,
        "getHeight",
        true,
        &[]
    ).unwrap().i().unwrap() as f32;

    [x, y, width, height]
}

pub fn world_to_screen(entity_pos: [f64; 3]) -> [f32; 2] {
    let entity_pos = entity_pos.map(|c| c as f32);
    let entity_pos_vec = Vec3::new(entity_pos[0], entity_pos[1], entity_pos[2]);

    let view_projection_matrix = get_view_projection_matrix();
    let mut result = view_projection_matrix * Vec4::new(entity_pos_vec.x, entity_pos_vec.y, entity_pos_vec.z, 1.0_f32);
    result = result / result.w;
    [result.x, result.y]
}

pub fn get_camera_direction(pitch: f32, yaw: f32) -> Vec3 {
    let pitch = pitch.to_radians();
    let yaw = yaw.to_radians();

    Vec3::new(
        pitch.cos() * yaw.cos(),
        pitch.sin(),
        pitch.cos() * yaw.sin()
    )
}

pub fn get_view_projection_matrix() -> Mat4 {
    let aspect = 1920.0_f32 / 1080.0_f32;
    let proj = perspective(aspect, 90.0_f32.to_radians(), 0.1_f32, 1000.0_f32);
    let cam_look_vec = get_camera_direction(0.0_f32, 0.0_f32);
    let view = look_at(&Vec3::new(0.0_f32, 0.0_f32, 0.0_f32), &cam_look_vec, &Vec3::new(0.0_f32, 1.0_f32, 0.0_f32));
    proj * view
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

pub fn render_outline<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, minecraft_client: &'a ClassMapping<'a>, xy: [i32; 2], width: i32, height: i32, color: i32) {
    let gui_graphics = make_gui_graphics(env, mappings_manager, minecraft_client);
    call_method_or_get_field!(
        env,
        gui_graphics,
        "renderOutline",
        false,
        &[
            JValue::Int(xy[0]),
            JValue::Int(xy[1]),
            JValue::Int(width),
            JValue::Int(height),
            JValue::Int(color)
        ]
    ).unwrap();
}

pub fn make_gui_graphics<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager, minecraft_client: &'a ClassMapping<'a>) -> &'a ClassMapping<'a> {
    let render_buffers = mappings_manager.get("RenderBuffers").unwrap();
    apply_object!(
        render_buffers,
        call_method_or_get_field!(env, minecraft_client, "renderBuffers", false).unwrap().l().unwrap()
    );

    let buffer_source = mappings_manager.get("BufferSource").unwrap();
    apply_object!(
        buffer_source,
        call_method_or_get_field!(env, render_buffers, "bufferSource", false, &[]).unwrap().l().unwrap()
    );

    let gui_graphics = mappings_manager.get("GuiGraphics").unwrap();
    apply_object!(
        gui_graphics,
        call_method_or_get_field!(
            ctor
            env,
            gui_graphics,
            "<init>",
            &[
                JValue::Object(minecraft_client.get_object().unwrap()),
                JValue::Object(buffer_source.get_object().unwrap())
            ]
        ).unwrap()
    );

    gui_graphics
}
