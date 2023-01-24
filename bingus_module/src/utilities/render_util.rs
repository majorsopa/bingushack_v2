use glu_sys::{glMatrixMode, GL_PROJECTION, GL_MODELVIEW, glPushMatrix, glLoadMatrixf, GLfloat, glPopMatrix};

use crate::crate_prelude::*;

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
    let mut modelview = [0.0; 16];

    for i in 0..4 {
        for j in 0..4 {
            let field_name = format!("m{}{}", i, j);
            let field = call_method_or_get_field!(
                env,
                matrix_class_mapping,
                field_name.as_str(),
                true
            ).unwrap().f().unwrap();

            modelview[i * 4 + j] = field;
        }
    }

    modelview
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

pub fn get_render_pos_vec<'a>(env: JNIEnv<'a>, mappings_manager: &'a MappingsManager) -> [f32; 3] {
    let vec3d = mappings_manager.get("Vec3d").unwrap();
    let entity_renderer = mappings_manager.get("EntityRenderer").unwrap();
}
