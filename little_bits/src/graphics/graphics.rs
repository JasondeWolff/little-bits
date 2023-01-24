extern crate glfw;
use glfw::{Action, Context, Key, Glfw, Window, WindowEvent};

use crate::maths::*;
use crate::system::*;
use crate::resources::Model;
use crate::application::*;
use crate::app;
use crate::Shared;

use std::mem;
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

pub extern crate imgui;

#[path = "opengl/opengl.rs"] pub mod opengl;
use opengl::*;

#[path = "opencl/opencl.rs"] pub mod opencl;
use opencl::*;

pub mod camera;
pub use camera::*;

pub struct GLModel {
    pub meshes: Vec<GLMesh>,
    pub materials: Vec<GLMaterial>,
}

#[derive(PartialEq, Clone, Debug, Copy)]
pub struct ModelInstance {
    pub transform: Transform
    // Material etc
}

pub struct Graphics {
    glfw: Glfw,
    window: Window,
    window_events: Receiver<(f64, WindowEvent)>,

    cl_context: CLContext,

    pub(crate) imgui: ImGui,

    render_camera: Shared<Camera>,
    dynamic_models: HashMap<*const Model, (GLModel, Vec<Shared<ModelInstance>>)>,
    shader_program: GLShaderProgram
}

impl System for Graphics {
    fn init() -> Box<Self> {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let default_dimensions = Int2::new(1280, 720);

        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::Samples(Some(4)));
        
        let (mut window, events) = glfw.create_window(default_dimensions.x as u32, default_dimensions.y as u32, "Little Bits", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_all_polling(true);
        window.make_current();
        glfw.set_swap_interval(glfw::SwapInterval::Sync(0));

        gl_init(&mut window);
        gl_enable_depth();
        gl_cull(gl::BACK);

        let cl_context = CLContext::new(&mut window);

        let vertex_shader_src = app().resources().get_text(String::from("assets/shaders/vert.glsl"));
        let vertex_shader = GLShader::new(GLShaderType::VERTEX, &vertex_shader_src.as_ref());
        let fragment_shader_src = app().resources().get_text(String::from("assets/shaders/frag.glsl"));
        let fragment_shader = GLShader::new(GLShaderType::FRAGMENT, &fragment_shader_src.as_ref());

        let shader_program = GLShaderProgram::new(&vertex_shader, &fragment_shader);

        let mut imgui = ImGui::new();
        imgui.resize(default_dimensions);

        Box::new(Graphics {
            glfw: glfw,
            window: window,
            window_events: events,
            cl_context: cl_context,
            imgui: imgui,
            render_camera: Shared::empty(),
            dynamic_models: HashMap::new(),
            shader_program: shader_program
        })
    }

    fn update(&mut self) {
        self.pre_render();
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

    pub fn set_icon(&mut self, icon: Shared<Image>) {
        let icon = icon.as_ref();
        assert_eq!(icon.channel_count, 4, "Failed to set icon. (Icon image must contain 4 channels)");
        
        let pixels: Vec<u32> = unsafe { mem::transmute(icon.data.clone()) };

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

    pub(crate) fn debug_ui(&mut self) -> &mut DebugUI {
        self.imgui.new_frame()
    }

    pub fn create_camera(&mut self) -> Shared<Camera> {
        let camera = Shared::new(Camera::new());
        camera
    }

    pub fn set_render_camera(&mut self, camera: Shared<Camera>) {
        self.render_camera = camera;
    }

    pub fn create_dynamic_model_instance(&mut self, model: Shared<Model>, transform: Option<Transform>) -> Shared<ModelInstance> {
        let model_ptr = model.as_ptr();

        let transform = match transform {
            Some(transform) => transform,
            None => Transform::new()
        };

        let model_instance = Shared::new(ModelInstance {
            transform: transform
        });

        match self.dynamic_models.get_mut(&model_ptr) {
            Some(models) => {
                models.1.push(model_instance.clone());
            },
            None => {
                let mut meshes: Vec<GLMesh> = Vec::new();
                for mesh in model.as_ref().meshes.iter() {
                    meshes.push(GLMesh::new(mesh));
                }

                let mut materials: Vec<GLMaterial> = Vec::new();
                for material in model.as_ref().materials.iter() {
                    materials.push(GLMaterial::new(material.clone()));
                }

                let gl_model = GLModel {
                    meshes: meshes,
                    materials: materials
                };

                self.dynamic_models.insert(model_ptr, (gl_model, vec![model_instance.clone()]));
            }
        }

        model_instance
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
                app().graphics().resize(Int2::new(width, height));
            }
            _ => {}
        }
    }

    fn resize(&mut self, dimensions: Int2) {
        gl_viewport(dimensions);
        self.imgui.resize(dimensions);
    }

    fn pre_render(&mut self) {

    }

    fn render(&mut self) {
        gl_clear_color(Float3::new(0.1, 0.1, 0.1));
        gl_clear();

        let (proj, view, view_pos) = match self.render_camera.try_as_mut() {
            Some(mut camera) => {
                (camera.get_proj_matrix(), camera.get_view_matrix(), camera.get_translation())
            },
            None => {
                let aspect_ratio: f32 = self.dimensions().x as f32 / self.dimensions().y as f32;
                let proj = Float4x4::perspective(60.0, aspect_ratio, 0.01, 1000.0);
                let view = Float4x4::identity();
                let view_pos = Float3::default();

                (proj, view, view_pos)
            }
        };

        for (_, models) in self.dynamic_models.iter_mut() {
            for model_transform in models.1.iter_mut() {
                let materials = &models.0.materials;

                for mesh in models.0.meshes.iter() {
                    self.shader_program.bind(); {
                        let mut model_transform = model_transform.as_mut();
                        self.shader_program.set_float4x4(&String::from("model"), model_transform.transform.get_matrix());
                        self.shader_program.set_float4x4(&String::from("projection"), proj);
                        self.shader_program.set_float4x4(&String::from("view"), view);

                        self.shader_program.set_float3(&String::from("viewPos"), view_pos);
                    
                        let material = &materials[mesh.material_idx()];
                        material.bind(&mut self.shader_program);

                        mesh.draw();
                    } self.shader_program.unbind();
                }
            }
        }

        self.imgui.render();
        //self.imgui.new_frame();
        self.window.swap_buffers();
    }

    fn post_render(&mut self) {
        for (_, models) in self.dynamic_models.iter_mut() {
            let mut indices = Vec::new();

            for (i, model_transform) in models.1.iter().rev().enumerate() {
                if model_transform.strong_count() == 1 {
                    indices.push(i);
                }
            }

            vec_remove_multiple(&mut models.1, &mut indices);
        }
    }
}

fn vec_remove_multiple<T>(vec: &mut Vec<T>, indices: &mut Vec<usize>) {
    indices.sort();    

    let mut j: usize = 0;
    for i in indices.iter() {
        vec.remove(i - j);
        j += 1;
    }
}