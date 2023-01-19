use gl::types::GLuint;

use super::{compile_shader, link_program};

static VS_SRC: &'static str = "
#version 150
in vec2 position;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}";
static FS_SRC: &'static str = "
#version 150
out vec4 out_color;
void main() {
    out_color = vec4(1.0, 1.0, 1.0, 1.0);
}";

pub fn compile_triangle() -> GLuint {
    let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    link_program(vs, fs)
}
