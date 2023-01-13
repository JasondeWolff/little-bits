extern crate gl;

extern crate glfw;
use glfw::Window;

use std::{mem, collections::HashMap};
pub use std::ffi::*;
pub use crate::maths::*;

pub fn gl_init(window: &mut Window) {
    gl::load_with(|s| window.get_proc_address(s) as *const _);
    gl_check();

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
	    //gl::Enable(gl::CULL_FACE);
	    //gl::CullFace(gl::BACK);
        gl_check();
    }
}

pub fn gl_check() {
    unsafe {
        let error = gl::GetError();
        match error {
            gl::NO_ERROR => return,
            gl::INVALID_ENUM => panic!("GL invalid enum."),
            gl::INVALID_VALUE => panic!("GL invalid value."),
            gl::INVALID_OPERATION => panic!("GL invalid operation."),
            gl::OUT_OF_MEMORY => panic!("GL out of memory."),
            gl::STACK_OVERFLOW => panic!("GL stack overflow."),
            gl::STACK_UNDERFLOW => panic!("GL stack underflow"),
            _ => panic!("GL unkown error."),
        }
    }
}

/*****************************************************************************
*                               COMMON
******************************************************************************/

pub fn gl_clear_color(color: Float3) {
    unsafe {
        gl::ClearColor(color.x, color.y, color.z, 1.0f32);
        gl_check();
    }
}

pub fn gl_clear() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl_check();
    }
}

pub fn gl_viewport(dimensions: Int2) {
    unsafe {
        gl::Viewport(0, 0, dimensions.x, dimensions.y);
        gl_check();
    }
}

/*****************************************************************************
*                               BUFFERS
******************************************************************************/

pub type GLBuffer = gl::types::GLuint;

pub fn gl_gen_buffer() -> GLBuffer {
    let mut buffer: u32 = 0;
    unsafe {
        gl::GenBuffers(1, &mut buffer);
        gl_check();
    }
    buffer
}

pub fn gl_gen_vert_array() -> GLBuffer {
    let mut buffer: u32 = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut buffer);
        gl_check();
    }
    buffer
}

pub fn gl_del_buffer(buffer: GLBuffer) {
    unsafe {
        gl::DeleteBuffers(1, &buffer);
        gl_check();
    }
}

pub fn gl_del_vert_array(buffer: GLBuffer) {
    unsafe {
        gl::DeleteVertexArrays(1, &buffer);
        gl_check();
    }
}

pub fn gl_vertex_attrib_ptr(index: u32, size: usize, stride: usize, ptr: *const c_void) {
    unsafe {
        gl::VertexAttribPointer(index, size as i32, gl::FLOAT, gl::FALSE, stride as i32, ptr);
        gl_check();
    }
}

pub fn gl_enable_vertex_attrib_array(index: u32) {
    unsafe {
        gl::EnableVertexAttribArray(index);
        gl_check();
    }
}

pub trait IGLBuffer {
    fn new() -> Self;

    fn bind(&self);
    fn unbind(&self);

    fn set_data(&self, size: usize, data: *mut c_void);
}

pub struct GLVAO {
    buffer: GLBuffer
}

impl IGLBuffer for GLVAO {
    fn new() -> Self {
        GLVAO {
            buffer: gl_gen_vert_array()
        }
    }

    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.buffer);
            gl_check();
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
            gl_check();
        }
    }

    fn set_data(&self, size: usize, data: *mut c_void) {
        panic!("Failed to set data on VAO.")
    }
}

impl Drop for GLVAO {
    fn drop(&mut self) {
        gl_del_vert_array(self.buffer);
    }
}

pub struct GLVBO {
    buffer: GLBuffer
}

impl IGLBuffer for GLVBO {
    fn new() -> Self {
        GLVBO {
            buffer: gl_gen_buffer()
        }
    }

    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.buffer);
            gl_check();
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl_check();
        }
    }

    fn set_data(&self, size: usize, data: *mut c_void) {
        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER, size as isize, data, gl::STATIC_DRAW);
            gl_check();
        }
    }
}

impl Drop for GLVBO {
    fn drop(&mut self) {
        gl_del_buffer(self.buffer);
    }
}

pub struct GLEBO {
    buffer: GLBuffer
}

impl IGLBuffer for GLEBO {
    fn new() -> Self {
        GLEBO {
            buffer: gl_gen_buffer()
        }
    }

    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.buffer);
            gl_check();
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl_check();
        }
    }

    fn set_data(&self, size: usize, data: *mut c_void) {
        unsafe {
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size as isize, data, gl::STATIC_DRAW);
            gl_check();
        }
    }
}

impl Drop for GLEBO {
    fn drop(&mut self) {
        gl_del_buffer(self.buffer);
    }
}

/*****************************************************************************
*                               SHADERS
******************************************************************************/

pub type GLShaderBuffer = gl::types::GLuint;
pub type GLShaderProgramBuffer = gl::types::GLuint;

pub fn gl_create_vert_shader() -> GLShaderBuffer {
    unsafe {
        let shader: GLShaderBuffer = gl::CreateShader(gl::VERTEX_SHADER);
        gl_check();
        shader
    }
}

pub fn gl_create_frag_shader() -> GLShaderBuffer {
    unsafe {
        let shader: GLShaderBuffer = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl_check();
        shader
    }
}

pub fn gl_shader_source(shader: GLShaderBuffer, source: &String) {
    let mut safe_source = source.clone();
    safe_source.push('\0');

    unsafe {
        let source_ptr_ptr = &(safe_source.as_ptr()) as *const *const u8;
        gl::ShaderSource(shader, 1, source_ptr_ptr as *const *const gl::types::GLchar, std::ptr::null());
        gl_check();
    }
}

pub fn gl_compile_shader(shader: GLShaderBuffer) {
    unsafe {
        gl::CompileShader(shader);
        gl_check();

        if cfg!(debug_assertions) {
            let mut buffer_data: [u8; 1024] = [0; 1024];
            let mut info_size: usize = 0;

            gl::GetShaderInfoLog(shader, (mem::size_of::<char>() * buffer_data.len()) as i32, (&mut info_size) as *mut usize as *mut i32, buffer_data.as_mut_ptr() as *mut c_char);
            gl_check();

            let mut buffer_str: String = String::new();
            for i in 0..info_size {
                buffer_str.push(buffer_data[i] as char);
            }

            if (info_size > 0 && buffer_str.contains("error")) {
               panic!("Failed to compile shader. \nOpenGL Error:\n{}\n", buffer_str);
            }
        }
    }
}

pub fn gl_attach_shader(shader: GLShaderBuffer, shader_program: GLShaderProgramBuffer) {
    unsafe {
        gl::AttachShader(shader_program, shader);
        gl_check();
    }
}

pub fn gl_del_shader(shader: GLShaderBuffer) {
    unsafe {
        gl::DeleteShader(shader);
        gl_check();
    }
}

pub fn gl_create_program() -> GLShaderProgramBuffer {
    unsafe {
        let program = gl::CreateProgram();
        gl_check();
        program
    }
}

pub fn gl_link_program(shader_program: GLShaderProgramBuffer) {
    unsafe {
        gl::LinkProgram(shader_program);
        gl_check();

        if cfg!(debug_assertions) {
            let mut buffer_data: [u8; 1024] = [0; 1024];
            let mut info_size: usize = 0;

            gl::GetProgramInfoLog(shader_program, (mem::size_of::<char>() * buffer_data.len()) as i32, (&mut info_size) as *mut usize as *mut i32, buffer_data.as_mut_ptr() as *mut c_char);
            gl_check();

            let mut buffer_str: String = String::new();
            for i in 0..info_size {
                buffer_str.push(buffer_data[i] as char);
            }

            if (info_size > 0 && buffer_str.contains("error")) {
               panic!("Failed to link program. \nOpenGL Error:\n{}\n", buffer_str);
            }
        }
    }
}

pub fn gl_use_program(shader_program: GLShaderProgramBuffer) {
    unsafe {
        gl::UseProgram(shader_program);
        gl_check();
    }
}

pub struct GLShader {
    buffer: GLShaderBuffer
}

pub enum GLShaderType {
    VERTEX,
    FRAGMENT
}

impl GLShader {
    pub fn new(shader_type: GLShaderType, source: &String) -> Self {
        let buffer: GLShaderBuffer = match shader_type {
            GLShaderType::VERTEX => gl_create_vert_shader(),
            GLShaderType::FRAGMENT => gl_create_frag_shader()
        };

        gl_shader_source(buffer, source);
        gl_compile_shader(buffer);

        GLShader {
            buffer: buffer
        }
    }

    pub fn attach(&self, shader_program: &GLShaderProgram) {
        gl_attach_shader(self.buffer, shader_program.buffer());
    }
}

impl Drop for GLShader {
    fn drop(&mut self) {
        gl_del_shader(self.buffer);
    }
}

pub struct GLShaderProgram {
    buffer: GLShaderProgramBuffer,
    uniform_locations: HashMap<String, i32>,
    uniforms: Option<Vec<GLUniform>>
}

#[derive(Clone)]
pub struct GLUniform {
    name: String,
    sort: u32,
    size: i32
}

impl GLShaderProgram {
    pub fn new(vertex_shader: &GLShader, fragment_shader: &GLShader) -> GLShaderProgram {
        let program = GLShaderProgram {
            buffer: gl_create_program(),
            uniform_locations: HashMap::new(),
            uniforms: None
        };

        vertex_shader.attach(&program);
        fragment_shader.attach(&program);

        gl_link_program(program.buffer);

        program
    }

    pub fn buffer(&self) -> GLShaderProgramBuffer {
        self.buffer
    }

    pub fn bind(&self) {
        gl_use_program(self.buffer);
    }

    pub fn unbind(&self) {
        gl_use_program(0);
    }

    pub fn uniforms(&mut self) -> Vec<GLUniform> {
        match &self.uniforms {
            Some(uniforms) => uniforms.clone(),
            None => {
                let mut count: i32 = 0;
                unsafe {
                    gl::GetProgramiv(self.buffer, gl::ACTIVE_ATTRIBUTES, &mut count as *mut i32);
                    gl_check();
                }

                let mut uniforms: Vec<GLUniform> = Vec::new();

                for i in 0..count {
                    let mut name_data: [u8; 1024] = [0; 1024];
                    let mut name_length: i32 = 0;

                    let mut uniform_size: i32 = 0;
                    let mut uniform_type: u32 = 0;

                    unsafe {
                        gl::GetActiveUniform(self.buffer, i as u32, 1024, (&mut name_length) as *mut i32, (&mut uniform_size) as *mut i32, (&mut uniform_type) as *mut u32, name_data.as_mut_ptr() as *mut c_char);
                        gl_check();
                    }

                    let mut name_str: String = String::new();
                    for i in 0..name_length {
                        name_str.push(name_data[i as usize] as char);
                    }

                    uniforms.push(GLUniform {
                        name: name_str,
                        sort: uniform_type,
                        size: uniform_size
                    })
                }

                self.uniforms = Some(uniforms);
                self.uniforms.clone().unwrap()
            }
        }
    }

    pub fn set_int(&mut self, name: &String, value: i32) {
        unsafe {
            gl::Uniform1i(self.uniform_location(name), value);
            gl_check();
        }
    }

    pub fn set_float(&mut self, name: &String, value: f32) {
        unsafe {
            gl::Uniform1f(self.uniform_location(name), value);
            gl_check();
        }
    }

    pub fn set_float4x4(&mut self, name: &String, value: Float4x4) {
        unsafe {
            gl::UniformMatrix4fv(self.uniform_location(name), 1, gl::FALSE, value.elems.as_ptr() as *const f32);
            gl_check();
        }
    }

    fn uniform_location(&mut self, name: &String) -> i32 {
        match self.uniform_locations.get(name) {
            Some(location) => location.clone(),
            None => {
                unsafe {
                    let mut cname = name.clone();
                    cname.push('\0');
                    
                    let location: i32 = gl::GetUniformLocation(self.buffer, cname.as_ptr() as *const i8);
                    gl_check();
                    assert!(location >= 0, "Failed to get uniform location. (Name: '{}')", name);

                    self.uniform_locations.insert(name.clone(), location);
                    location
                }
            }
        }
    }
}

/*****************************************************************************
*                               DRAWING
******************************************************************************/

pub fn gl_draw_elems(mode: gl::types::GLenum, count: usize, index_type: gl::types::GLenum) {
    unsafe {
        gl::DrawElements(mode, count as i32, index_type, std::ptr::null());
        gl_check();
    }
}