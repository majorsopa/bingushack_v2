use glam::{Vec4, Vec3, Vec2};
use glu_sys::{glMatrixMode, GL_PROJECTION, GL_MODELVIEW, glPushMatrix, glLoadMatrixf, GLfloat, glPopMatrix};

use crate::crate_prelude::*;


pub struct RenderInfo {
    entity_pos: [f64; 3],
    bounding_box: [f64; 6],
}

impl RenderInfo {
    pub fn new(entity_pos: [f64; 3], bounding_box: [f64; 6]) -> Self {
        Self {
            entity_pos,
            bounding_box
        }
    }
}

pub fn setup_ortho(projection_matrix: *const GLfloat, modelview_matrix: *const GLfloat) {
    unsafe {
        glPushMatrix();
        glMatrixMode(GL_PROJECTION);
        glLoadMatrixf(projection_matrix);
        glMatrixMode(GL_MODELVIEW);
        glLoadMatrixf(modelview_matrix);
    }
}

pub fn restore_gl() {
    unsafe {
        glPopMatrix();
    }
}

pub fn get_viewport<'a>(env: JNIEnv<'a>, viewport_class_mapping: &'a ClassMapping<'a>) -> [i32; 4] {
    let x = call_method_or_get_field!(
        env,
        viewport_class_mapping,
        "x",
        true
    ).unwrap().i().unwrap();
    let y = call_method_or_get_field!(
        env,
        viewport_class_mapping,
        "y",
        true
    ).unwrap().i().unwrap();
    let width = call_method_or_get_field!(
        env,
        viewport_class_mapping,
        "width",
        true
    ).unwrap().i().unwrap();
    let height = call_method_or_get_field!(
        env,
        viewport_class_mapping,
        "height",
        true
    ).unwrap().i().unwrap();

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

pub fn get_render_system<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager) -> &'a ClassMapping<'a> {
    mappings_manager.get("RenderSystem").unwrap()
}

pub fn get_viewport_class_mapping<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager) -> &'a ClassMapping<'a> {
    mappings_manager.get("Viewport").unwrap()
}

pub fn get_modelview_class_mapping<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager) -> &'a ClassMapping<'a> {
    let matrix = mappings_manager.get("Matrix4f").unwrap();
    let render_system = get_render_system(env, mappings_manager);

    apply_object!(
        matrix,
        call_method_or_get_field!(
            env,
            render_system,
            "getModelViewMatrix",
            true,
            &[]
        ).unwrap().l().unwrap()
    );

    matrix
}

pub fn get_projection_class_mapping<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager) -> &'a ClassMapping<'a> {
    let matrix = mappings_manager.get("Matrix4f").unwrap();
    let render_system = get_render_system(env, mappings_manager);

    apply_object!(
        matrix,
        call_method_or_get_field!(
            env,
            render_system,
            "getProjectionMatrix",
            true,
            &[]
        ).unwrap().l().unwrap()
    );

    matrix
}

pub fn world_to_screen_multiply(in_vec: Vec4, mat: [GLfloat; 16]) -> Vec4 {
    let [x, y, z, w] = in_vec.to_array();
    Vec4::new(
        x * mat[0] + y * mat[4] + z * mat[8] + w * mat[12],
        x * mat[1] + y * mat[5] + z * mat[9] + w * mat[13],
        x * mat[2] + y * mat[6] + z * mat[10] + w * mat[14],
        x * mat[3] + y * mat[7] + z * mat[11] + w * mat[15],
    )
}

pub fn world_to_screen(point_in_world: Vec3, model_view: [GLfloat; 16], projection: [GLfloat; 16], viewport: [GLfloat; 4]) -> Option<Vec2> {
    let clip_space_pos = world_to_screen_multiply(
        world_to_screen_multiply(
            Vec4::new(point_in_world.x, point_in_world.y, point_in_world.z, 1.0),
            model_view
        ),
        projection
    );
    // ncd = normalized device coordinates
    let ncd_space_pos = Vec3::new(
        clip_space_pos.x / clip_space_pos.w,
        clip_space_pos.y / clip_space_pos.w,
        clip_space_pos.z / clip_space_pos.w
    );

    if ncd_space_pos.z > 1.0 || ncd_space_pos.z < -1.0 {
        return None;
    } else {
        let screen_space_pos = Vec2::new(
            ((ncd_space_pos.x + 1.0) / 2.0) * viewport[2],
            ((1.0 - ncd_space_pos.y) / 2.0) * viewport[3]
        );

        Some(screen_space_pos)
    }
}
