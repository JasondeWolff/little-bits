use glfw::{Window, Context, Glfw};

extern crate cl_wrapper;
use cl_wrapper::*;

use crate::graphics::opengl::*;
use rand::Rng;

use crate::app;
use crate::graphics::camera::Camera;
use std::f32::consts::PI;

pub struct Baker {
    context: CLContext,
    command_queue: CLCommandQueue,
    program: CLProgram,
    kernel: CLKernel,
    shader_program: GLShaderProgram,

    display_shader_program: GLShaderProgram,
    display_vao: GLVAO
}

pub enum BakeSampleDistribution {
    Uniform,
    Random
}

pub struct BakeParameters {
    pub epochs: usize,
    pub sample_positions: usize,
    pub sample_distribution: BakeSampleDistribution,
    pub sample_resolution: usize
}

impl Default for BakeParameters {
    fn default() -> Self {
        BakeParameters {
            epochs: 100,
            sample_positions: 300,
            sample_distribution: BakeSampleDistribution::Random,
            sample_resolution: 1024
        }
    }
}

impl Baker {
    pub fn new(window: &mut Window) -> Self {
        let context = CLContext::new(window);
        let command_queue = CLCommandQueue::new(&context);
        let program_src = app().resources().get_text(String::from("assets/cl/bake.cl"));
        let program = CLProgram::new(&context, &program_src.as_ref(), Some(&String::from("assets/cl/")));
        let kernel = CLKernel::new(&program, &String::from("render"));

        let shader_program;
        {
            let vertex_shader_src = app().resources().get_text(String::from("assets/shaders/vert.glsl"));
            let vertex_shader = GLShader::new(GLShaderType::VERTEX, &vertex_shader_src.as_ref());
            let fragment_shader_src = app().resources().get_text(String::from("assets/shaders/bake_frag.glsl"));
            let fragment_shader = GLShader::new(GLShaderType::FRAGMENT, &fragment_shader_src.as_ref());
            shader_program = GLShaderProgram::new(&vertex_shader, &fragment_shader);
        }

        let display_shader_program;
        let display_vao = GLVAO::new();
        {
            let vertex_shader_src = app().resources().get_text(String::from("assets/shaders/quad_vert.glsl"));
            let vertex_shader = GLShader::new(GLShaderType::VERTEX, &vertex_shader_src.as_ref());
            let fragment_shader_src = app().resources().get_text(String::from("assets/shaders/quad_frag.glsl"));
            let fragment_shader = GLShader::new(GLShaderType::FRAGMENT, &fragment_shader_src.as_ref());
            display_shader_program = GLShaderProgram::new(&vertex_shader, &fragment_shader);
        }

        Baker {
            context: context,
            command_queue: command_queue,
            program: program,
            kernel: kernel,
            shader_program: shader_program,
            display_shader_program: display_shader_program,
            display_vao: display_vao
        }
    }

    // Source: https://www.cmu.edu/biolphys/deserno/pdf/sphere_equi.pdf
    fn uniform_sphere_points(point_count: usize, radius: f32) -> Vec<Float3> {
        let mut points = Vec::with_capacity(point_count);

        let mut n_count = 0;
        let a = (4.0 * PI * radius * radius) / point_count as f32;
        let d = a.sqrt();
        let mv = (PI / d).round();
        let dv = PI / mv;
        let dphi = a / dv;

        for m in 0..(mv as usize) {
            let v = PI * (m as f32 + 0.5) / mv;
            let mphi = (2.0 * PI * (v / dphi).sin()).round();
            for n in 0..(mphi as usize) {
                let phi = (2.0 * PI * n as f32) / mphi;

                let x = v.sin() * phi.cos();
                let y = v.sin() * phi.sin();
                let z = v.cos();
                points.push(Float3::new(x, y, z));

                n_count += 1;
                if n_count >= point_count {
                    return points;
                }
            }
        }

        points
    }

    fn random_sphere_points(point_count: usize, radius: f32) -> Vec<Float3> {
        let mut points = Vec::with_capacity(point_count);
        let mut rng = rand::thread_rng();

        for _ in 0..point_count {
            let z = rng.gen_range(-radius, radius);
            let phi = rng.gen_range(0.0, 2.0 * PI);

            let omz_sqr = (radius * radius - z * z).sqrt();
            let x = omz_sqr * phi.cos();
            let y = omz_sqr * phi.sin();

            points.push(Float3::new(x, y, z));
        }

        points
    }

    pub fn bake(&mut self, model: &GLModel, params: &BakeParameters, window: &mut Window, glfw: &mut Glfw) {
        let (min, max) = model.bounds();
        let radius = (max - min).magnitude() * 0.5;
        let center = (max - min) * 0.5 + min;

        let mut camera = Camera::new();
        camera.set_aspect_ratio(Some(1.0));
        let camera_points = match params.sample_distribution {
            BakeSampleDistribution::Random => Self::random_sphere_points(params.sample_positions, radius),
            BakeSampleDistribution::Uniform => Self::uniform_sphere_points(params.sample_positions, radius)
        };

        assert!(params.sample_resolution > 1, "Failed to bake nemo. (Sample resolution must be 2 or larger)");

        let base_color_rt = GLRenderTexture::new(params.sample_resolution, params.sample_resolution);
        let cl_base_color = CLGLTexture2D::new(&self.context, base_color_rt.tex(), CLBufferMode::Read);

        let normal_rt = GLRenderTexture::new(params.sample_resolution, params.sample_resolution);
        let cl_normal = CLGLTexture2D::new(&self.context, normal_rt.tex(), CLBufferMode::Read);

        let mro_rt = GLRenderTexture::new(params.sample_resolution, params.sample_resolution);
        let cl_mro = CLGLTexture2D::new(&self.context, mro_rt.tex(), CLBufferMode::Read);

        let emission_rt = GLRenderTexture::new(params.sample_resolution, params.sample_resolution);
        let cl_emission = CLGLTexture2D::new(&self.context, emission_rt.tex(), CLBufferMode::Read);

        let mut render_target = GLRenderTarget::new(params.sample_resolution, params.sample_resolution);
        render_target.set_texture(GLRenderAttachment::Color(0), base_color_rt);
        render_target.set_texture(GLRenderAttachment::Color(1), normal_rt);
        render_target.set_texture(GLRenderAttachment::Color(2), mro_rt);
        render_target.set_texture(GLRenderAttachment::Color(3), emission_rt);
        render_target.check();

        let display_target = GLRenderTexture::new(app().graphics().dimensions().x as usize, app().graphics().dimensions().y as usize);
        let cl_display_target = CLGLTexture2D::new(&self.context, display_target.tex(), CLBufferMode::Write);

        for e in 0..params.epochs {
            for camera_point in &camera_points {
                glfw.poll_events();

                camera.set_translation(-camera_point);
                camera.set_rotation(Quaternion::look_rotation(center - camera_point, Float3::up()));

                // Render inputs to rt's
                gl_viewport(Int2::new(params.sample_resolution as i32, params.sample_resolution as i32));
                {
                    render_target.bind(); {
                        gl_clear();

                        let materials = &model.materials;

                        for mesh in model.meshes.iter() {
                            self.shader_program.bind(); {
                                self.shader_program.set_float4x4(&String::from("model"), Float4x4::identity());
                                self.shader_program.set_float4x4(&String::from("projection"), camera.get_proj_matrix());
                                self.shader_program.set_float4x4(&String::from("view"), camera.get_view_matrix());
                            
                                let material = &materials[mesh.material_idx()];
                                material.bind(&mut self.shader_program);

                                mesh.draw();
                            } self.shader_program.unbind();
                        }
                    }  render_target.unbind();
                }

                // Train nemo
                gl_finish();
                {
                    // Acquire gl resources
                    self.command_queue.acquire_gl_texture(&cl_base_color);
                    self.command_queue.acquire_gl_texture(&cl_normal);
                    self.command_queue.acquire_gl_texture(&cl_mro);
                    self.command_queue.acquire_gl_texture(&cl_emission);
                    self.command_queue.acquire_gl_texture(&cl_display_target);

                    self.kernel.set_arg_buffer(0, &cl_display_target);
                    self.kernel.set_arg_buffer(1, &cl_base_color);
                    self.kernel.set_arg_buffer(2, &cl_normal);
                    self.kernel.set_arg_buffer(3, &cl_mro);
                    self.kernel.set_arg_buffer(4, &cl_emission);

                    self.command_queue.execute(&self.kernel, &vec![app().graphics().dimensions().x as usize, app().graphics().dimensions().y as usize], None);
                    self.command_queue.finish();
                    
                    // Release gl resources
                    self.command_queue.release_gl_texture(&cl_display_target);
                    self.command_queue.release_gl_texture(&cl_emission);
                    self.command_queue.release_gl_texture(&cl_mro);
                    self.command_queue.release_gl_texture(&cl_normal);
                    self.command_queue.release_gl_texture(&cl_base_color);
                }

                // Display rt result
                gl_viewport(app().graphics().dimensions());
                {
                    gl_clear();

                    self.display_shader_program.bind(); {
                        let t = &display_target;//render_target.get_texture(GLRenderAttachment::Color(0)).unwrap();
                        t.bind(0);
                        self.display_shader_program.set_sampler_slot(&String::from("tex"), 0);

                        self.display_vao.bind(); {
                            gl_draw_arrays(gl::TRIANGLES, 0, 6);
                        } self.display_vao.unbind();
                    } self.display_shader_program.unbind();
                }

                window.swap_buffers();

                //std::thread::sleep(std::time::Duration::from_millis(500));
            }

            println!("EPOCHS: [{} / {}]", e, params.epochs);
        }
    }
}