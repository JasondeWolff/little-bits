use glfw::{Window, Context, Glfw};

extern crate cl_wrapper;
use cl_wrapper::*;

use crate::graphics::opengl::*;
use rand::Rng;

use crate::{app, Timer};
use crate::graphics::camera::*;
use std::f32::consts::PI;

struct MultiHashGrid {
    meta: MultiHashGridMeta,
    meta_buffer: CLBuffer,
    elems: Vec<f32>,
    elem_buffer_readonly: CLBuffer,
    elem_buffer_writeonly: CLBuffer
}

#[repr(C)]
struct MultiHashGridMeta {
    resolution_layers: i32,
    max_entries: i32,
    features_per_entry: i32,
    min_resolution: i32,
    max_resolution: i32,
    width: f32,
    height: f32,
    depth: f32,
}

#[repr(C)]
struct AABB {
    low: Float3,
    _0: f32,
    high: Float3,
    _1: f32
}

impl AABB {
    pub fn new(low: Float3, high: Float3) -> Self {
        AABB {
            low: low,
            _0: 0.0,
            high: high,
            _1: 0.0
        }
    }
}

impl MultiHashGrid {
    pub fn new(cl_context: &CLContext, resolution_layers: usize, max_entries: usize, features_per_entry: usize, min_resolution: usize, max_resolution: usize, size: Float3) -> Self {
        let mut elems = Vec::with_capacity(resolution_layers * max_entries * features_per_entry);
        let mut rng = rand::thread_rng();
        for _ in 0..elems.capacity() {
            elems.push(rng.gen_range(-0.0001, 0.0001));
        }

        MultiHashGrid {
            meta: MultiHashGridMeta {
                resolution_layers: resolution_layers as i32,
                max_entries: max_entries as i32,
                features_per_entry: features_per_entry as i32,
                min_resolution: min_resolution as i32,
                max_resolution: max_resolution as i32,
                width: size.x,
                height: size.y,
                depth: size.z
            },
            meta_buffer: CLBuffer::new(cl_context, CLBufferMode::Read, std::mem::size_of::<MultiHashGridMeta>()),
            elems: elems,
            elem_buffer_readonly: CLBuffer::new(cl_context, CLBufferMode::Read, resolution_layers * max_entries * features_per_entry * 4),
            elem_buffer_writeonly: CLBuffer::new(cl_context, CLBufferMode::Write, resolution_layers * max_entries * features_per_entry * 4)
        }
    }

    pub fn write(&mut self, cl_command_queue: &CLCommandQueue) {
        cl_command_queue.write_buffer(&self.meta_buffer, &mut self.meta as *mut MultiHashGridMeta as *mut c_void);
        cl_command_queue.write_buffer(&self.elem_buffer_readonly, self.elems.as_mut_ptr() as *mut c_void);
        cl_command_queue.write_buffer(&self.elem_buffer_writeonly, self.elems.as_mut_ptr() as *mut c_void);
    }

    pub fn read(&mut self, cl_command_queue: &CLCommandQueue) {
        cl_command_queue.read_buffer(&self.elem_buffer_writeonly, self.elems.as_mut_ptr() as *mut c_void);
    }

    fn fix_nan(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.elems.len() {
            if f32::is_nan(self.elems[i]) {
                self.elems[i] = rng.gen_range(-0.0001, 0.0001);
            }
        }
    }

    pub fn set_kernel_arg(&mut self, cl_kernel: &CLKernel, idx: u32) -> u32 {
        cl_kernel.set_arg_buffer(idx + 0, &self.meta_buffer);
        cl_kernel.set_arg_buffer(idx + 1, &self.elem_buffer_readonly);
        cl_kernel.set_arg_buffer(idx + 2, &self.elem_buffer_writeonly);
        idx + 3
    }

    pub fn required_nn_inputs(&self) -> usize {
        (self.meta.resolution_layers * self.meta.features_per_entry) as usize
    }
}

struct NeuralNetwork {
    input_count: i32,
    hidden_count: i32,
    output_count: i32,
    hidden_layer_count: i32,

    weights: Vec<f32>
}

fn weight_init(rng: &mut rand::ThreadRng, nj: i32, nj1: i32) -> f32 {
    let sqrt6 = 2.44948974278f32; // (6.0).sqrt()
    let range = sqrt6 / (nj as f32 + nj1 as f32).sqrt();
    rng.gen_range(-range, range)
}

impl NeuralNetwork {
    fn new(input_count: i32, hidden_count: i32, output_count: i32, hidden_layer_count: i32) -> Self {
        let weight_count = input_count * hidden_count + hidden_count * hidden_count * hidden_layer_count + hidden_count * output_count;
        let bias_count = hidden_count * hidden_layer_count + output_count;
        let mut weights = Vec::with_capacity((weight_count + bias_count) as usize);

        let mut rng = rand::thread_rng();
        for _ in 0..(input_count * hidden_count) {
            weights.push(weight_init(&mut rng, input_count, hidden_count));
        }
        for _ in (input_count * hidden_count)..(input_count * hidden_count + hidden_count * hidden_count * hidden_layer_count) {
            weights.push(weight_init(&mut rng, hidden_count, hidden_count));
        }
        for _ in (input_count * hidden_count + hidden_count * hidden_count * hidden_layer_count)..(input_count * hidden_count + hidden_count * hidden_count * hidden_layer_count + hidden_count * output_count) {
            weights.push(weight_init(&mut rng, hidden_count, output_count));
        }

        for _ in 0..bias_count {
            weights.push(rng.gen_range(-0.01, 0.01));
        }
        
        NeuralNetwork {
            input_count: input_count,
            hidden_count: hidden_count,
            output_count: output_count,
            hidden_layer_count: hidden_layer_count,
            weights: weights
        }
    }

    fn required_cache_size(&self) -> usize {
        (self.input_count + self.hidden_count * self.hidden_layer_count + self.output_count) as usize * 2 * 4
    }

    fn fix_nan(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..(self.input_count * self.hidden_count) as usize {
            if f32::is_nan(self.weights[i]) {
                self.weights[i] = weight_init(&mut rng, self.input_count, self.hidden_count);
            }
        }
        for i in (self.input_count * self.hidden_count) as usize..(self.input_count * self.hidden_count + self.hidden_count * self.hidden_count * self.hidden_layer_count) as usize {
            if f32::is_nan(self.weights[i]) {
                self.weights[i] = weight_init(&mut rng, self.hidden_count, self.hidden_count);
            }
        }
        for i in (self.input_count * self.hidden_count + self.hidden_count * self.hidden_count * self.hidden_layer_count) as usize..(self.input_count * self.hidden_count + self.hidden_count * self.hidden_count * self.hidden_layer_count + self.hidden_count * self.output_count) as usize {
            if f32::is_nan(self.weights[i]) {
                self.weights[i] = weight_init(&mut rng, self.hidden_count, self.output_count);
            }
        }
    }
}

struct CLNeuralNetwork {
    input_count: i32,
    hidden_count: i32,
    output_count: i32,
    hidden_layer_count: i32
}

impl CLNeuralNetwork {
    pub fn new(nn: &NeuralNetwork) -> Self {
        CLNeuralNetwork {
            input_count: nn.input_count,
            hidden_count: nn.hidden_count,
            output_count: nn.output_count,
            hidden_layer_count: nn.hidden_layer_count
        }
    }
}

pub struct Baker {
    context: CLContext,
    command_queue: CLCommandQueue,
    program: CLProgram,
    kernel: CLKernel,
    train_kernel: CLKernel,
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
            epochs: 10000,
            sample_positions: 300,
            sample_distribution: BakeSampleDistribution::Random,
            sample_resolution: 512
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
        let train_kernel = CLKernel::new(&program, &String::from("train"));

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
            train_kernel: train_kernel,
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
        assert!(params.sample_resolution > 1, "Failed to bake nemo. (Sample resolution must be 2 or larger)");

        let (mut min, mut max) = model.bounds();
        min *= 1.5;
        max *= 1.5;

        let size = max - min;
        let radius = size.magnitude() * 0.5;
        let center = (max + min) * 0.5;
        let mut aabb = AABB::new(min, max);

        let mut camera = Camera::new();
        camera.set_aspect_ratio(Some(1.0));
        camera.set_fov(90.0);
        let camera_points = match params.sample_distribution {
            BakeSampleDistribution::Random => Self::random_sphere_points(params.sample_positions, radius * 1.5),
            BakeSampleDistribution::Uniform => Self::uniform_sphere_points(params.sample_positions, radius * 1.5)
        };

        let camera_points = vec![Float3::new(radius * 1.5, 0.0, 0.0)];

        let position_rt = GLRenderTexture::new(params.sample_resolution, params.sample_resolution);
        let cl_position = CLGLTexture2D::new(&self.context, position_rt.tex(), CLBufferMode::Read);
        let base_color_rt = GLRenderTexture::new(params.sample_resolution, params.sample_resolution);
        let cl_base_color = CLGLTexture2D::new(&self.context, base_color_rt.tex(), CLBufferMode::Read);
        let normal_rt = GLRenderTexture::new(params.sample_resolution, params.sample_resolution);
        let cl_normal = CLGLTexture2D::new(&self.context, normal_rt.tex(), CLBufferMode::Read);
        let mro_rt = GLRenderTexture::new(params.sample_resolution, params.sample_resolution);
        let cl_mro = CLGLTexture2D::new(&self.context, mro_rt.tex(), CLBufferMode::Read);
        let emission_rt = GLRenderTexture::new(params.sample_resolution, params.sample_resolution);
        let cl_emission = CLGLTexture2D::new(&self.context, emission_rt.tex(), CLBufferMode::Read);

        let mut render_target = GLRenderTarget::new(params.sample_resolution, params.sample_resolution);
        render_target.set_texture(GLRenderAttachment::Color(0), position_rt);
        render_target.set_texture(GLRenderAttachment::Color(1), base_color_rt);
        render_target.set_texture(GLRenderAttachment::Color(2), normal_rt);
        render_target.set_texture(GLRenderAttachment::Color(3), mro_rt);
        render_target.set_texture(GLRenderAttachment::Color(4), emission_rt);
        render_target.check();

        let display_target = GLRenderTexture::new(params.sample_resolution, params.sample_resolution);
        let cl_display_target = CLGLTexture2D::new(&self.context, display_target.tex(), CLBufferMode::Write);

        let cl_camera = CLBuffer::new(&self.context, CLBufferMode::Read, std::mem::size_of::<CLCamera>());

        // Hey future me, trilinear filtering is probably flipped!
        let mut multi_hash_grid = MultiHashGrid::new(&self.context, 16, 2usize.pow(22), 1, 128, 512, size);

        let mut neural_network = NeuralNetwork::new(multi_hash_grid.required_nn_inputs() as i32 + 1, 32, 3, 2);
        println!("Using {}B per kernel", neural_network.required_cache_size());
        let mut cl_nn_rep = CLNeuralNetwork::new(&neural_network);
        let cl_neural_network = CLBuffer::new(&self.context, CLBufferMode::Read, std::mem::size_of::<CLNeuralNetwork>());
        let cl_in_weights = CLBuffer::new(&self.context, CLBufferMode::Read, std::mem::size_of::<f32>() * neural_network.weights.len());
        let cl_out_weights = CLBuffer::new(&self.context, CLBufferMode::Write, std::mem::size_of::<f32>() * neural_network.weights.len());

        let cl_aabb = CLBuffer::new(&self.context, CLBufferMode::Read, std::mem::size_of::<AABB>());
        let cl_loss = CLBuffer::new(&self.context, CLBufferMode::Write, std::mem::size_of::<f32>());
        let cl_errors = CLBuffer::new(&self.context, CLBufferMode::ReadWrite, std::mem::size_of::<f32>() * (multi_hash_grid.required_nn_inputs() + 1));

        let mut timer = Timer::new();

        for e in 0..params.epochs {
            if (e % 10 == 0) 
            {
                let br = 0;
            }

            for camera_point in &camera_points {
                glfw.poll_events();

                // Render inputs to rt's
                gl_viewport(Int2::new(params.sample_resolution as i32, params.sample_resolution as i32));
                {
                    render_target.bind(); {
                        gl_clear_color(Float3::new(0.0, 0.0, 0.0));
                        gl_clear();

                        let materials = &model.materials;

                        for mesh in model.meshes.iter() {
                            self.shader_program.bind(); {
                                self.shader_program.set_float4x4(&String::from("model"), Float4x4::identity());
                                self.shader_program.set_float4x4(&String::from("projection"), camera.get_proj_matrix());
                                self.shader_program.set_float4x4(&String::from("view"), Float4x4::look_at(camera_point.clone(), center, Float3::up()));
                                self.shader_program.set_float3(&String::from("viewPos"), camera_point.clone());

                                let material = &materials[mesh.material_idx()];
                                material.bind(&mut self.shader_program);

                                mesh.draw();
                            } self.shader_program.unbind();
                        }
                    }  render_target.unbind();
                }

                let mut cl_camera_rep = CLCamera::new(-camera_point, &(center - camera_point).normalized(), 90.0, 1.0);

                // Train nemo
                gl_finish();
                {
                    // Acquire gl resources
                    self.command_queue.acquire_gl_texture(&cl_position);
                    self.command_queue.acquire_gl_texture(&cl_base_color);
                    self.command_queue.acquire_gl_texture(&cl_normal);
                    self.command_queue.acquire_gl_texture(&cl_mro);
                    self.command_queue.acquire_gl_texture(&cl_emission);
                    self.command_queue.acquire_gl_texture(&cl_display_target);

                    self.command_queue.write_buffer(&cl_camera, &mut cl_camera_rep as *mut CLCamera as *mut c_void);
                    self.command_queue.write_buffer(&cl_neural_network, &mut cl_nn_rep as *mut CLNeuralNetwork as *mut c_void);
                    self.command_queue.write_buffer(&cl_in_weights, neural_network.weights.as_mut_ptr() as *mut c_void);
                    self.command_queue.write_buffer(&cl_out_weights, neural_network.weights.as_mut_ptr() as *mut c_void);
                    multi_hash_grid.write(&self.command_queue);
                    self.command_queue.write_buffer(&cl_aabb, &mut aabb as *mut AABB as *mut c_void);
                    let mut zero = 0.0f32;
                    self.command_queue.write_buffer(&cl_loss, &mut zero as *mut f32 as *mut c_void);
                    let mut zeros = vec![0.0f32; multi_hash_grid.required_nn_inputs() + 1];
                    self.command_queue.write_buffer(&cl_errors, zeros.as_mut_ptr() as *mut c_void);

                    self.kernel.set_arg_buffer(0, &cl_display_target);
                    self.kernel.set_arg_buffer(1, &cl_position);
                    self.kernel.set_arg_buffer(2, &cl_base_color);
                    self.kernel.set_arg_buffer(3, &cl_normal);
                    self.kernel.set_arg_buffer(4, &cl_mro);
                    self.kernel.set_arg_buffer(5, &cl_emission);
                    self.kernel.set_arg_buffer(6, &cl_camera);
                    self.kernel.set_arg_buffer(7, &cl_neural_network);
                    self.kernel.set_arg_buffer(8, &cl_in_weights);
                    self.kernel.set_arg_buffer(9, &cl_out_weights);
                    self.kernel.set_arg_empty(10, neural_network.required_cache_size());
                    self.kernel.set_arg_int(11, (neural_network.required_cache_size() / 4) as i32);
                    multi_hash_grid.set_kernel_arg(&self.kernel, 12);
                    self.kernel.set_arg_buffer(15, &cl_aabb);
                    self.kernel.set_arg_buffer(16, &cl_loss);
                    self.kernel.set_arg_buffer(17, &cl_errors);

                    let local_work_dims = vec![1, 1];
                    self.command_queue.execute(&self.kernel, &vec![display_target.width() as usize, display_target.height() as usize], Some(&local_work_dims));
                    self.command_queue.finish();

                    self.train_kernel.set_arg_buffer(0, &cl_position);
                    self.train_kernel.set_arg_buffer(1, &cl_camera);
                    self.train_kernel.set_arg_buffer(2, &cl_neural_network);
                    self.train_kernel.set_arg_buffer(3, &cl_in_weights);
                    self.train_kernel.set_arg_buffer(4, &cl_out_weights);
                    self.train_kernel.set_arg_empty(5, neural_network.required_cache_size());
                    self.train_kernel.set_arg_int(6, (neural_network.required_cache_size() / 4) as i32);
                    multi_hash_grid.set_kernel_arg(&self.train_kernel, 7);
                    self.train_kernel.set_arg_buffer(10, &cl_aabb);
                    self.train_kernel.set_arg_buffer(11, &cl_errors);
                    self.train_kernel.set_arg_float(12, timer.elapsed() as f32);

                    self.command_queue.execute(&self.train_kernel, &vec![display_target.width() as usize, display_target.height() as usize], Some(&local_work_dims));
                    self.command_queue.finish();
                    
                    self.command_queue.read_buffer(&cl_out_weights, neural_network.weights.as_mut_ptr());
                    //neural_network.fix_nan();
                    let mut loss = 0.0f32;
                    self.command_queue.read_buffer(&cl_loss, &mut loss as *mut f32);
                    println!("loss: {}", loss);
                    multi_hash_grid.read(&self.command_queue);
                    //multi_hash_grid.fix_nan();

                    // FIND A BETTER SOLUTION THAN FIXING NAN VALUES!! PREVENT THEM!!! (L2 regulization??)
                    // google for: weight explosion or nn divergence
                    // ON TOP OF THAT, MAKE RELU WORK
                    // ALSO CREATE SMALL 3x3 kernels which respect eachothers weights or errors?

                    // Release gl resources
                    self.command_queue.release_gl_texture(&cl_display_target);
                    self.command_queue.release_gl_texture(&cl_emission);
                    self.command_queue.release_gl_texture(&cl_mro);
                    self.command_queue.release_gl_texture(&cl_normal);
                    self.command_queue.release_gl_texture(&cl_base_color);
                    self.command_queue.release_gl_texture(&cl_position);
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
            }

            println!("EPOCHS: [{} / {}]", e, params.epochs);
        }
    }
}