use glfw::Window;

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
    shader_program: GLShaderProgram
}

pub enum BakeSampleDistribution {
    Uniform,
    Random
}

pub struct BakeParameters {
    pub epochs: usize,
    pub sample_positions: usize,
    pub sample_distribution: BakeSampleDistribution
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

    pub fn bake(&self, model: &GLModel, params: &BakeParameters) {
        let (min, max) = model.bounds();
        let radius = (max - min).magnitude() * 0.5;
        let center = (max - min) * 0.5 + min;

        let mut camera = Camera::new();
        let camera_points = match params.sample_distribution {
            BakeSampleDistribution::Random => Self::random_sphere_points(params.sample_positions, radius),
            BakeSampleDistribution::Uniform => Self::uniform_sphere_points(params.sample_positions, radius)
        };

        let base_color_rt = GLRenderTexture::new(1024, 1024, GLRenderAttachment::Color(0));
        base_color_rt.bind();
        let cl_base_color = CLGLTexture2D::new(&self.context, base_color_rt.tex(), CLBufferMode::Read);

        for _ in 0..params.epochs {
            for camera_point in &camera_points {
                camera.set_translation(*camera_point);
                camera.set_rotation(Quaternion::look_rotation(-camera_point, Float3::up()));


            }
        }

        base_color_rt.unbind();
    }
}