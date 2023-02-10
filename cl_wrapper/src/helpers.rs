extern crate cl3;
pub use cl3::types::*;
use std::ffi::{c_void, CString};

extern crate glfw;
use glfw::Window;

extern crate gl_wrapper;
use gl_wrapper::*;

use windows::Win32::Graphics::OpenGL::wglGetCurrentDC;

fn cl_check<T>(result: Result<T, cl_int>) -> T {
    match result {
        Ok(value) => value,
        Err(result) => {
            match result {
                //cl3::error_codes::CL_SUCCESS => return,
                cl3::error_codes::CL_DEVICE_NOT_FOUND => panic!("Error: CL_DEVICE_NOT_FOUND"),
                cl3::error_codes::CL_DEVICE_NOT_AVAILABLE => panic!("Error: CL_DEVICE_NOT_AVAILABLE"),
                cl3::error_codes::CL_COMPILER_NOT_AVAILABLE => panic!("Error: CL_COMPILER_NOT_AVAILABLE"),
                cl3::error_codes::CL_MEM_OBJECT_ALLOCATION_FAILURE => panic!("Error: CL_MEM_OBJECT_ALLOCATION_FAILURE"),
                cl3::error_codes::CL_OUT_OF_RESOURCES => panic!("Error: CL_OUT_OF_RESOURCES"),
                cl3::error_codes::CL_OUT_OF_HOST_MEMORY => panic!("Error: CL_OUT_OF_HOST_MEMORY"),
                cl3::error_codes::CL_PROFILING_INFO_NOT_AVAILABLE => panic!("Error: CL_PROFILING_INFO_NOT_AVAILABLE"),
                cl3::error_codes::CL_MEM_COPY_OVERLAP => panic!("Error: CL_MEM_COPY_OVERLAP"),
                cl3::error_codes::CL_IMAGE_FORMAT_MISMATCH => panic!("Error: CL_IMAGE_FORMAT_MISMATCH"),
                cl3::error_codes::CL_IMAGE_FORMAT_NOT_SUPPORTED => panic!("Error: CL_IMAGE_FORMAT_NOT_SUPPORTED"),
                cl3::error_codes::CL_BUILD_PROGRAM_FAILURE => panic!("Error: CL_BUILD_PROGRAM_FAILURE"),
                cl3::error_codes::CL_MAP_FAILURE => panic!("Error: CL_MAP_FAILURE"),
                cl3::error_codes::CL_MISALIGNED_SUB_BUFFER_OFFSET => panic!("Error: CL_MISALIGNED_SUB_BUFFER_OFFSET"),
                cl3::error_codes::CL_EXEC_STATUS_ERROR_FOR_EVENTS_IN_WAIT_LIST => panic!("Error: CL_EXEC_STATUS_ERROR_FOR_EVENTS_IN_WAIT_LIST"),
                cl3::error_codes::CL_INVALID_VALUE => panic!("Error: CL_INVALID_VALUE"),
                cl3::error_codes::CL_INVALID_DEVICE_TYPE => panic!("Error: CL_INVALID_DEVICE_TYPE"),
                cl3::error_codes::CL_INVALID_PLATFORM => panic!("Error: CL_INVALID_PLATFORM"),
                cl3::error_codes::CL_INVALID_DEVICE => panic!("Error: CL_INVALID_DEVICE"),
                cl3::error_codes::CL_INVALID_CONTEXT => panic!("Error: CL_INVALID_CONTEXT"),
                cl3::error_codes::CL_INVALID_QUEUE_PROPERTIES => panic!("Error: CL_INVALID_QUEUE_PROPERTIES"),
                cl3::error_codes::CL_INVALID_COMMAND_QUEUE => panic!("Error: CL_INVALID_COMMAND_QUEUE"),
                cl3::error_codes::CL_INVALID_HOST_PTR => panic!("Error: CL_INVALID_HOST_PTR"),
                cl3::error_codes::CL_INVALID_MEM_OBJECT => panic!("Error: CL_INVALID_MEM_OBJECT"),
                cl3::error_codes::CL_INVALID_IMAGE_FORMAT_DESCRIPTOR => panic!("Error: CL_INVALID_IMAGE_FORMAT_DESCRIPTOR"),
                cl3::error_codes::CL_INVALID_IMAGE_SIZE => panic!("Error: CL_INVALID_IMAGE_SIZE"),
                cl3::error_codes::CL_INVALID_SAMPLER => panic!("Error: CL_INVALID_SAMPLER"),
                cl3::error_codes::CL_INVALID_BINARY => panic!("Error: CL_INVALID_BINARY"),
                cl3::error_codes::CL_INVALID_BUILD_OPTIONS => panic!("Error: CL_INVALID_BUILD_OPTIONS"),
                cl3::error_codes::CL_INVALID_PROGRAM => panic!("Error: CL_INVALID_PROGRAM"),
                cl3::error_codes::CL_INVALID_PROGRAM_EXECUTABLE => panic!("Error: CL_INVALID_PROGRAM_EXECUTABLE"),
                cl3::error_codes::CL_INVALID_KERNEL_NAME => panic!("Error: CL_INVALID_KERNEL_NAME"),
                cl3::error_codes::CL_INVALID_KERNEL_DEFINITION => panic!("Error: CL_INVALID_KERNEL_DEFINITION"),
                cl3::error_codes::CL_INVALID_KERNEL => panic!("Error: CL_INVALID_KERNEL"),
                cl3::error_codes::CL_INVALID_ARG_INDEX => panic!("Error: CL_INVALID_ARG_INDEX"),
                cl3::error_codes::CL_INVALID_ARG_VALUE => panic!("Error: CL_INVALID_ARG_VALUE"),
                cl3::error_codes::CL_INVALID_ARG_SIZE => panic!("Error: CL_INVALID_ARG_SIZE"),
                cl3::error_codes::CL_INVALID_KERNEL_ARGS => panic!("Error: CL_INVALID_KERNEL_ARGS"),
                cl3::error_codes::CL_INVALID_WORK_DIMENSION => panic!("Error: CL_INVALID_WORK_DIMENSION"),
                cl3::error_codes::CL_INVALID_WORK_GROUP_SIZE => panic!("Error: CL_INVALID_WORK_GROUP_SIZE"),
                cl3::error_codes::CL_INVALID_WORK_ITEM_SIZE => panic!("Error: CL_INVALID_WORK_ITEM_SIZE"),
                cl3::error_codes::CL_INVALID_GLOBAL_OFFSET => panic!("Error: CL_INVALID_GLOBAL_OFFSET"),
                cl3::error_codes::CL_INVALID_EVENT_WAIT_LIST => panic!("Error: CL_INVALID_EVENT_WAIT_LIST"),
                cl3::error_codes::CL_INVALID_EVENT => panic!("Error: CL_INVALID_EVENT"),
                cl3::error_codes::CL_INVALID_OPERATION => panic!("Error: CL_INVALID_OPERATION"),
                cl3::error_codes::CL_INVALID_GL_OBJECT => panic!("Error: CL_INVALID_GL_OBJECT"),
                cl3::error_codes::CL_INVALID_BUFFER_SIZE => panic!("Error: CL_INVALID_BUFFER_SIZE"),
                cl3::error_codes::CL_INVALID_MIP_LEVEL => panic!("Error: CL_INVALID_MIP_LEVEL"),
                cl3::error_codes::CL_INVALID_GLOBAL_WORK_SIZE => panic!("Error: CL_INVALID_GLOBAL_WORK_SIZE"),
                _ => panic!("CL unkown error.")
            }
        }
    }
}

pub struct CLContext {
    device: cl_device_id,
    context: cl_context
}

impl CLContext {
    pub fn new(window: &mut Window) -> Self {
        let platform_ids = cl_check(cl3::platform::get_platform_ids());
        assert!(0 < platform_ids.len(), "Failed to init OpenCL. (No platforms found)");
        let platform_id = platform_ids[0];
    
        let device_ids = cl_check(cl3::device::get_device_ids(platform_id, cl3::device::CL_DEVICE_TYPE_ALL));
        assert!(0 < device_ids.len(), "Failed to init OpenCL. (No devices found)");
    
        let device_id: cl_device_id = || -> cl_device_id {
            for device_id in device_ids {
                let device_extensions = cl_check(cl3::device::get_device_info(device_id, cl3::device::CL_DEVICE_EXTENSIONS));
    
                let device_extensions_str: String = device_extensions.into();
                for device_extension_str in device_extensions_str.split_whitespace() {
                    if device_extension_str == "cl_khr_gl_sharing" {
                        return device_id;
                    }
                }
            }
            panic!("Failed to init OpenCL. (No device compatible with cl_khr_gl_sharing found)")
        }();
    
        let device_name = cl_check(cl3::device::get_device_info(device_id, cl3::device::CL_DEVICE_NAME));
        let device_name_str: String = device_name.into();
        let available_mem = cl_check(cl3::device::get_device_info(device_id, cl3::device::CL_DEVICE_GLOBAL_MEM_SIZE));
        let available_mem: u64 = available_mem.into();
        let local_available_mem = cl_check(cl3::device::get_device_info(device_id, cl3::device::CL_DEVICE_LOCAL_MEM_SIZE));
        let local_available_mem: u64 = local_available_mem.into();
        println!("OpenCL using device: {} ({}MB global, {}KB local)", device_name_str, available_mem / 1024 / 1024, local_available_mem / 1024);
    
        let context_properties: [cl_context_properties; 7] = unsafe {[
            cl3::context::CL_CONTEXT_PLATFORM, std::mem::transmute(platform_id),
            cl3::gl::CL_WGL_HDC_KHR, wglGetCurrentDC().0,//*(wglGetCurrentDC().0 as *mut isize)
            cl3::gl::CL_GL_CONTEXT_KHR, std::mem::transmute(window.get_wgl_context()),
            0
        ]};
        let context = cl_check(cl3::context::create_context(&[device_id], context_properties.as_ptr(), None, std::ptr::null_mut()));

        CLContext {
            device: device_id,
            context: context
        }
    }

    pub fn device_handle(&self) -> cl_device_id {
        self.device
    }

    pub fn context_handle(&self) -> cl_context {
        self.context
    }
}

impl Drop for CLContext {
    fn drop(&mut self) {
        unsafe {
            cl_check(cl3::context::release_context(self.context));
        }
    }
}

pub struct CLProgram {
    program: cl_program
}

impl CLProgram {
    pub fn new(context: &CLContext, source: &String, dir: Option<&String>) -> Self {
        let program = cl_check(cl3::program::create_program_with_source(context.context_handle(), &[source.as_str()]));

        let options = match dir {
            Some(dir) => {
                CString::new(format!("-I {}", dir)).unwrap()
            },
            None => {
                CString::new("").unwrap()
            }
        };

        match cl3::program::build_program(program, &[context.device_handle()], options.as_c_str(), None, std::ptr::null_mut()) {
            Err(_) => {
                let log = cl_check(cl3::program::get_program_build_info(program, context.device_handle(), cl3::program::CL_PROGRAM_BUILD_LOG));
                let log: String = log.into();

                panic!("Failed to build CLProgram. \nError:\n\n {}", log);
            },
            Ok(_) => {}
        }

        CLProgram {
            program: program
        }
    }

    pub fn handle(&self) -> cl_program {
        self.program
    }
}

impl Drop for CLProgram {
    fn drop(&mut self) {
        unsafe {
            cl_check(cl3::program::release_program(self.program));
        }
    }
}

pub struct CLKernel {
    kernel: cl_kernel
}

impl CLKernel {
    pub fn new(program: &CLProgram, name: &String) -> Self {
        let mut name = name.clone();
        name.push('\0');
        let cname = unsafe { CString::from_raw(name.as_mut_ptr() as *mut i8) };
        std::mem::forget(name);

        let kernel = cl_check(cl3::kernel::create_kernel(program.handle(), cname.as_c_str()));

        CLKernel {
            kernel: kernel
        }
    }

    pub fn handle(&self) -> cl_kernel {
        self.kernel
    }

    pub fn set_arg_buffer(&self, idx: u32, buffer: &dyn ICLMem) {
        unsafe {
            let buffer_ptr_ptr: *mut *mut c_void = &mut buffer.handle();
            cl_check(cl3::kernel::set_kernel_arg(self.kernel, idx, std::mem::size_of::<cl_mem>(), std::mem::transmute(buffer_ptr_ptr)));
        }
    }

    pub fn set_arg_int(&self, idx: u32, value: i32) {
        unsafe {
            cl_check(cl3::kernel::set_kernel_arg(self.kernel, idx, std::mem::size_of::<i32>(), &value as *const i32 as *const c_void));
        }
    }
}

impl Drop for CLKernel {
    fn drop(&mut self) {
        unsafe {
            cl_check(cl3::kernel::release_kernel(self.kernel));
        }
    }
}

pub struct CLCommandQueue {
    command_queue: cl_command_queue
}

impl CLCommandQueue {
    pub fn new(context: &CLContext) -> Self {
        let command_queue = unsafe {
            cl_check(cl3::command_queue::create_command_queue(context.context_handle(), context.device_handle(), 0))
        };

        CLCommandQueue {
            command_queue: command_queue
        }
    }

    pub fn handle(&self) -> cl_command_queue {
        self.command_queue
    }

    pub fn execute(&self, kernel: &CLKernel, global_work_dims: &Vec<usize>, local_work_dims: Option<&Vec<usize>>) {
        unsafe {
            if let Some(local_work_dims) = local_work_dims {
                assert_eq!(global_work_dims.len(), local_work_dims.len(), "Failed to execute command queue. (Global and local work dims must match)");

                cl_check(cl3::command_queue::enqueue_nd_range_kernel(self.command_queue, kernel.handle(), global_work_dims.len() as u32, std::ptr::null(), global_work_dims.as_ptr() as *const usize, local_work_dims.as_ptr() as *const usize, 0, std::ptr::null()));
            } else {
                cl_check(cl3::command_queue::enqueue_nd_range_kernel(self.command_queue, kernel.handle(), global_work_dims.len() as u32, std::ptr::null(), global_work_dims.as_ptr() as *const usize, std::ptr::null(), 0, std::ptr::null()));
            }
        }
    }

    pub fn finish(&self) {
        cl_check(cl3::command_queue::finish(self.command_queue));
    }

    pub fn acquire_gl_texture(&self, texture: &CLGLTexture2D) {
        unsafe {
            let texture_ptr_ptr: *mut *mut c_void = &mut texture.handle();
            cl_check(cl3::gl::enqueue_acquire_gl_objects(self.command_queue, 1, std::mem::transmute(texture_ptr_ptr), 0, std::ptr::null()));
        }
    }

    pub fn release_gl_texture(&self, texture: &CLGLTexture2D) {
        unsafe {
            let texture_ptr_ptr: *mut *mut c_void = &mut texture.handle();
            cl_check(cl3::gl::enqueue_release_gl_objects(self.command_queue, 1, std::mem::transmute(texture_ptr_ptr), 0, std::ptr::null()));
        }
    }

    pub fn write_buffer(&self, buffer: &CLBuffer, data: *mut c_void) {
        unsafe {
            cl_check(cl3::command_queue::enqueue_write_buffer(self.command_queue, buffer.handle(), CL_FALSE, 0, buffer.size(), data, 0, std::ptr::null_mut()));
        }
    }

    pub fn read_buffer<T>(&self, buffer: &CLBuffer, data: &mut T) {
        unsafe {
            cl_check(cl3::command_queue::enqueue_read_buffer(self.command_queue, buffer.handle(), CL_FALSE, 0, buffer.size(), [data].as_mut_ptr() as *mut u8 as *mut c_void, 0, std::ptr::null()));
        }
    }
}

impl Drop for CLCommandQueue {
    fn drop(&mut self) {
        unsafe {
            cl_check(cl3::command_queue::release_command_queue(self.command_queue));
        }
    }
}

pub trait ICLMem {
    fn handle(&self) -> cl_mem;
}

pub enum CLBufferMode {
    Read,
    Write,
    ReadWrite
}

pub struct CLBuffer {
    mem: cl_mem,
    size: usize
}

impl ICLMem for CLBuffer {
    fn handle(&self) -> cl_mem {
        self.mem
    }
}

impl CLBuffer {
    pub fn new(context: &CLContext, mode: CLBufferMode, size: usize) -> Self {
        let buffer = unsafe {
            let flags = match mode {
                CLBufferMode::Read => cl3::memory::CL_MEM_READ_ONLY,
                CLBufferMode::Write => cl3::memory::CL_MEM_WRITE_ONLY,
                CLBufferMode::ReadWrite => cl3::memory::CL_MEM_READ_WRITE
            };
            cl_check(cl3::memory::create_buffer(context.context_handle(), flags, size, std::ptr::null_mut()))
        };

        CLBuffer {
            mem: buffer,
            size: size
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl Drop for CLBuffer {
    fn drop(&mut self) {
        unsafe {
            cl_check(cl3::memory::release_mem_object(self.mem));
        }
    }
}

pub struct CLGLTexture2D {
    mem: cl_mem,
}

impl ICLMem for CLGLTexture2D {
    fn handle(&self) -> cl_mem {
        self.mem
    }
}

impl CLGLTexture2D {
    pub fn new(context: &CLContext, gl_texture: &GLTexture, mode: CLBufferMode) -> Self {
        let buffer = unsafe {
            let flags = match mode {
                CLBufferMode::Read => cl3::memory::CL_MEM_READ_ONLY,
                CLBufferMode::Write => cl3::memory::CL_MEM_WRITE_ONLY,
                CLBufferMode::ReadWrite => cl3::memory::CL_MEM_READ_WRITE
            };
            cl_check(cl3::gl::create_from_gl_texture(context.context_handle(), flags, gl_texture.target(), 0, gl_texture.handle()))
        };

        CLGLTexture2D {
            mem: buffer,
        }
    }

    pub fn handle(&self) -> cl_mem {
        self.mem
    }
}

impl Drop for CLGLTexture2D {
    fn drop(&mut self) {
        unsafe {
            cl_check(cl3::memory::release_mem_object(self.mem));
        }
    }
}
