extern crate glfw;
use glfw::{Action, Context, Key, Glfw, Window, WindowEvent};

use crate::maths::*;
use crate::system::*;
use crate::resources::Model;
use crate::application::*;
use crate::app;
use crate::HandleQueue;
use crate::Shared;

use std::mem;
use std::slice;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::mpsc::Receiver;
use std::collections::HashMap;

/*
Setup:
 - ImGui
 - Scene Graph
 - Materials

Neural Mesh:
 - Neural network compute shader
 - Back propagation
 - Serialize and deserialize result

 Triangle Mesh Preview:
 - PBR Shading
 - Basic phong lights

 Multiple Neural Meshes:
  - BVH builder and traversal
*/

#[path = "opengl/opengl.rs"] pub mod opengl;
use opengl::*;

pub mod camera;
pub use camera::*;

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct ModelInstance(u64);

pub struct Graphics {
    glfw: Glfw,
    window: Window,
    window_events: Receiver<(f64, WindowEvent)>,

    render_camera: Shared<Camera>,

    dynamic_models: HashMap<*const Model, (Vec<GLMesh>, Vec<Transform>)>,
    dynamic_model_handles: HashMap<ModelInstance, *mut Transform>,
    dynamic_model_handle_queue: HandleQueue,

    shader_program: GLShaderProgram
}

impl System for Graphics {
    fn init() -> Box<Self> {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        
        let (mut window, events) = glfw.create_window(1280, 720, "Little Bits", glfw::WindowMode::Windowed)
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
            render_camera: Shared::empty(),
            dynamic_models: HashMap::new(),
            dynamic_model_handles: HashMap::new(),
            dynamic_model_handle_queue: HandleQueue::new(10000),
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
    pub fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    pub fn set_icon(&mut self, icon: &Image) {
        assert_eq!(icon.channel_count, 4, "Failed to set icon. (Icon image must contain 4 channels)");

        let pixels: Vec<u8> = unsafe { slice::from_raw_parts(icon.data, (icon.dimensions.x * icon.dimensions.y * icon.channel_count) as usize).to_vec() };
        let pixels: Vec<u32> = unsafe { mem::transmute(pixels) };

        let image: glfw::PixelImage = glfw::PixelImage {
            width: icon.dimensions.x as u32,
            height: icon.dimensions.y as u32,
            pixels: pixels
        };
        self.window.set_icon_from_pixels(vec![image]);
    }

    pub fn set_cursor_lock(&mut self, locked: bool) {
        if locked {
            self.window.set_cursor_mode(glfw::CursorMode::Disabled);
        } else {
            self.window.set_cursor_mode(glfw::CursorMode::Normal);
        }
    }

    pub fn dimensions(&self) -> Int2 {
        let (x, y) = self.window.get_size();
        Int2::new(x, y)
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn create_camera(&mut self) -> Shared<Camera> {
        let camera = Shared::new(Camera::new());
        camera
    }

    pub fn set_render_camera(&mut self, camera: Shared<Camera>) {
        self.render_camera = camera;
    }

    pub fn create_dynamic_model_instance(&mut self, model: Rc<Model>, transform: Option<Transform>) -> ModelInstance {
        let model_ptr = Rc::as_ptr(&model);

        let transform = match transform {
            Some(transform) => transform,
            None => Transform::new()
        };

        let handle = ModelInstance(self.dynamic_model_handle_queue.create());

        match self.dynamic_models.get_mut(&model_ptr) {
            Some(models) => {
                models.1.push(transform);
            },
            None => {
                let mut meshes: Vec<GLMesh> = Vec::new();
                for mesh in model.meshes.iter() {
                    meshes.push(GLMesh::new(mesh));
                }

                self.dynamic_models.insert(model_ptr, (meshes, vec![transform]));
            }
        }

        match self.dynamic_models.get_mut(&model_ptr) {
            Some(models) => {
                let transform_ptr = models.1.last_mut().unwrap() as *mut Transform;
                self.dynamic_model_handles.insert(handle.clone(), transform_ptr);
            },
            None => panic!("Failed to create dynamic model instance.")
        }

        handle
    }

    pub fn get_dynamic_model_transform(&mut self, model_instance: ModelInstance) -> &mut Transform {
        match self.dynamic_model_handles.get(&model_instance) {
            Some(transform) => {
                unsafe { (*transform).as_mut().unwrap() }
            },
            None => panic!("Failed to get dynamic model transform. (Invalid model instance)")
        }
    }

    pub fn destroy_dynamic_models(&mut self) {
        for (handle, _) in self.dynamic_model_handles.iter() {
            self.dynamic_model_handle_queue.destroy(handle.0);
        }
        self.dynamic_models.clear();
        self.dynamic_model_handles.clear();
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

        let (proj, view) = match self.render_camera.try_as_mut() {
            Some(mut camera) => {
                (camera.get_proj_matrix(), camera.get_view_matrix())
            },
            None => {
                let aspect_ratio: f32 = self.dimensions().x as f32 / self.dimensions().y as f32;
                let proj = Float4x4::perspective(60.0, aspect_ratio, 0.01, 1000.0);
                let view = Float4x4::identity();

                (proj, view)
            }
        };

        for (_, models) in self.dynamic_models.iter_mut() {
            for model_transform in models.1.iter_mut() {
                for mesh in models.0.iter() {
                    self.shader_program.bind(); {
                        self.shader_program.set_float4x4(&String::from("model"), model_transform.get_matrix());
                        self.shader_program.set_float4x4(&String::from("projection"), proj);
                        self.shader_program.set_float4x4(&String::from("view"), view);
                    
                        mesh.draw();
                    } self.shader_program.unbind();
                }
            }
        }

        self.window.swap_buffers();
    }

    fn post_render(&mut self) {
        
    }
}