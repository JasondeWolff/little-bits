extern crate cl3;
pub use cl3::types::*;

use windows::Win32::Graphics::OpenGL::wglGetCurrentDC;

extern crate glfw;
use glfw::Window;

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
        println!("OpenCL using device: {}", device_name_str);
    
        let context_properties: [cl_context_properties; 7] = unsafe {[
            cl3::context::CL_CONTEXT_PLATFORM, std::mem::transmute(platform_id),
            cl3::gl::CL_WGL_HDC_KHR, wglGetCurrentDC().0,//*(wglGetCurrentDC().0 as *mut isize)
            cl3::gl::CL_GL_CONTEXT_KHR, std::mem::transmute(window.get_wgl_context()),
            0
        ]};
        let context = cl_check(cl3::context::create_context(&[device_id], context_properties.as_ptr(), None, std::ptr::null_mut()));

        CLContext {
            context: context
        }
    }
}

impl Drop for CLContext {
    fn drop(&mut self) {
        unsafe {
            cl_check(cl3::context::release_context(self.context));
        }
    }
}