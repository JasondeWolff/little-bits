use glfw::Window;

extern crate cl_wrapper;
use cl_wrapper::*;

#[path = "opengl/opengl.rs"] mod opengl;
use opengl::*;

use crate::app;
use crate::graphics::camera::Camera;

pub struct Baker {
    context: CLContext,
    command_queue: CLCommandQueue,
    program: CLProgram,
    kernel: CLKernel,
    shader_program: GLShaderProgram
}

impl Baker {
    pub fn new(window: &mut Window) -> Self {
        let context = CLContext::new(window);
        let command_queue = CLCommandQueue::new(&context);
        let program_src = app().resources().get_text(String::from("assets/cl/bake.cl"));
        let program = CLProgram::new(&context, &program_src.as_ref());
        let kernel = CLKernel::new(&program, &String::from("render"));

        let vertex_shader_src = app().resources().get_text(String::from("assets/shaders/vert.glsl"));
        let vertex_shader = GLShader::new(GLShaderType::VERTEX, &vertex_shader_src.as_ref());
        let fragment_shader_src = app().resources().get_text(String::from("assets/shaders/frag.glsl"));
        let fragment_shader = GLShader::new(GLShaderType::FRAGMENT, &fragment_shader_src.as_ref());
        let shader_program = GLShaderProgram::new(&vertex_shader, &fragment_shader);

        Baker {
            context: context,
            command_queue: command_queue,
            program: program,
            kernel: kernel,
            shader_program: shader_program
        }
    }

    pub fn bake(&self, model: GLModel) {
        let camera = Camera::new();
        let (min, max) = model.bounds();

        
    }
}