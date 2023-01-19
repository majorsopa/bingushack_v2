use std::{mem, ptr, ffi::CString};

use gl::types::{GLuint, GLsizeiptr, GLfloat, GLboolean};
use once_cell::sync::OnceCell;

use crate::crate_prelude::{*, triangle::compile_triangle};

static ESP_SHADER: OnceCell<GLuint> = OnceCell::new();
static VERTEX_DATA: [GLfloat; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];


fn render(_esp: &mut Esp) {
    let shader = *ESP_SHADER.get_or_init(|| compile_triangle());




    let mut vao = 0;
    let mut vbo = 0;


    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);  // crash


        
        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&VERTEX_DATA[0]),
            gl::STATIC_DRAW,
        );


        // Use shader program
        gl::UseProgram(shader);
        let out_color_str = CString::new("out_color").unwrap();
        let out_color_str_ptr = out_color_str.as_ptr();
        gl::BindFragDataLocation(shader, 0, out_color_str_ptr);
        
        // Specify the layout of the vertex data
        let pos_str = CString::new("position").unwrap();
        let pos_str_ptr = pos_str.as_ptr();
        let pos_attr = gl::GetAttribLocation(shader, pos_str_ptr);
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(
            pos_attr as GLuint,
            2,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            0,
            ptr::null(),
        );
    }

    unsafe {
        // Clear the screen to a solid color
        gl::ClearColor(0.8, 0.1, 0.1, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        // Draw a triangle from the 3 vertices
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
    }
}


#[derive(BingusModuleTrait)]
#[add_bingus_fields]
#[bingus_module(name = "ESP", render_method = "render(self)")]
pub struct Esp {

}

impl MakeNewBingusModule for Esp {
    fn new() -> Self {
        Self {
            __enabled_bool_setting: (BingusSetting::BoolSetting(false.into()), "enabled"),
        }
    }
}