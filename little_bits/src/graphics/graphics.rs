extern crate glfw;
use glfw::{Action, Context, Key, Glfw, Window, WindowEvent};

use crate::maths::*;
use crate::system::*;
use crate::resources::Model;
use crate::application::*;
use crate::app;

use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::collections::HashMap;

#[path = "opengl/opengl.rs"] pub mod opengl;
use opengl::*;

pub struct Graphics {
    glfw: Glfw,
    window: Window,
    window_events: Receiver<(f64, WindowEvent)>,

    models: HashMap<*const Model, Vec<GLMesh>>,
    shader_program: GLShaderProgram
}

impl System for Graphics {
    fn init() -> Box<Self> {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        
        let (mut window, events) = glfw.create_window(1280, 720, "Rusterizer", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_all_polling(true);
        window.make_current();
        glfw.set_swap_interval(glfw::SwapInterval::Sync(0));

        gl_init(&mut window);
        gl_enable_depth();
        gl_cull(gl::BACK);

        let vertex_shader_src = app().resources().get_text(String::from("assets/shaders/vert.glsl"));
        let vertex_shader = GLShader::new(GLShaderType::VERTEX, &vertex_shader_src);
        let fragment_shader_src = app().resources().get_text(String::from("assets/shaders/frag.glsl"));
        let fragment_shader = GLShader::new(GLShaderType::FRAGMENT, &fragment_shader_src);

        let shader_program = GLShaderProgram::new(&vertex_shader, &fragment_shader);

        Box::new(Graphics {
            glfw: glfw,
            window: window,
            window_events: events,
            models: HashMap::new(),
            shader_program: shader_program
        })
    }

    fn update(&mut self) {
        self.render();
        self.post_render();

        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.window_events) {
            Graphics::handle_window_event(&mut self.window, event);
        }
    }
}

impl Graphics {
    pub fn dimensions(&self) -> Int2 {
        let (x, y) = self.window.get_size();
        Int2::new(x, y)
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn draw_model(&mut self, model: Rc<Model>) {
        let model_ptr = Rc::as_ptr(&model);
        match self.models.get(&model_ptr) {
            Some(_) => {
                // TODO: add model transforms
            },
            None => {
                let mut meshes: Vec<GLMesh> = Vec::new();
                for mesh in model.meshes.iter() {
                    meshes.push(GLMesh::new(mesh));
                }

                self.models.insert(model_ptr, meshes);
            }
        }
    }
}

impl Graphics {
    fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true);
            }

            glfw::WindowEvent::Key(key, _, action, _) => {
                let key_code: KeyCode = unsafe { std::mem::transmute(key as i16) };
                app().input().set_key(key_code, action != glfw::Action::Release);
            }
            glfw::WindowEvent::MouseButton(button, action, _) => {
                let button_code: MouseButton = unsafe { std::mem::transmute(button as i8) };
                app().input().set_button(button_code, action != glfw::Action::Release);
            }
            glfw::WindowEvent::CursorPos(x, y) => {
                app().input().set_mouse_position(Float2::new(x as f32, y as f32));
            }
            glfw::WindowEvent::FramebufferSize(width, height) => {
                Graphics::resize(width, height);
            }
            _ => {}
        }
    }

    fn resize(width: i32, height: i32) {
        gl_viewport(Int2::new(width, height));
    }

    fn render(&mut self) {
        gl_clear_color(Float3::new(1.0, 0.5, 0.32));
        gl_clear();

        let aspect_ratio: f32 = self.dimensions().x as f32 / self.dimensions().y as f32;
        let proj = Float4x4::perspective(60.0, aspect_ratio, 0.01, 1000.0);
        let view = Float4x4::identity();
        let model = Float4x4::translation(Float3::new(0.0, 0.0, -150.0)) * /*rotation */ Float4x4::scale(Float3::new(0.3, 0.3, 0.3));

        for (_, meshes) in self.models.iter() {
            for mesh in meshes.iter() {
                self.shader_program.bind(); {
                    self.shader_program.set_float4x4(&String::from("model"), model);
                    self.shader_program.set_float4x4(&String::from("projection"), proj);
                    self.shader_program.set_float4x4(&String::from("view"), view);
                
                    mesh.draw();
                } self.shader_program.unbind();
            }
        }

        self.window.swap_buffers();
    }

    fn post_render(&mut self) {
        self.models.clear();
    }
}