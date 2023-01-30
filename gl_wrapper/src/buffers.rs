use crate::*;

/*****************************************************************************
*                               STRUCTS
******************************************************************************/

pub type GLBuffer = gl::types::GLuint;

pub struct GLVAO {
    buffer: GLBuffer
}

pub struct GLVBO {
    buffer: GLBuffer
}

pub struct GLEBO {
    buffer: GLBuffer
}

pub struct GLFBO {
    buffer: GLBuffer
}


/*****************************************************************************
*                               FUNCS
******************************************************************************/

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

/*****************************************************************************
*                               IMPLEMENTATION
******************************************************************************/

pub trait IGLBuffer {
    fn new() -> Self;

    fn bind(&self);
    fn unbind(&self);

    fn set_data(&self, size: usize, data: *mut c_void);
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

    fn set_data(&self, _: usize, _: *mut c_void) {
        panic!("Failed to set data on VAO.")
    }
}

impl Drop for GLVAO {
    fn drop(&mut self) {
        gl_del_vert_array(self.buffer);
    }
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

impl IGLBuffer for GLFBO {
    fn new() -> Self {
        GLFBO {
            buffer: gl_gen_frame_buffer()
        }
    }

    fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.buffer);
            gl_check();

            assert_eq!(gl::CheckFramebufferStatus(gl::FRAMEBUFFER), gl::FRAMEBUFFER_COMPLETE, "Failed to bind frame buffer.");
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl_check();
        }
    }

    fn set_data(&self, _: usize, _: *mut c_void) {
        panic!("Failed to set data on FBO. (Should never be called)");
    }
}

impl Drop for GLFBO {
    fn drop(&mut self) {
        gl_del_frame_buffer(self.buffer);
    }
}

/*****************************************************************************
*                               HELPERS
******************************************************************************/

fn gl_gen_buffer() -> GLBuffer {
    let mut buffer: u32 = 0;
    unsafe {
        gl::GenBuffers(1, &mut buffer);
        gl_check();
    }
    buffer
}

fn gl_gen_vert_array() -> GLBuffer {
    let mut buffer: u32 = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut buffer);
        gl_check();
    }
    buffer
}

fn gl_gen_frame_buffer() -> GLBuffer {
    let mut buffer: u32 = 0;
    unsafe {
        gl::GenFramebuffers(1, &mut buffer);
        gl_check();
    }
    buffer
}

fn gl_del_buffer(buffer: GLBuffer) {
    unsafe {
        gl::DeleteBuffers(1, &buffer);
        gl_check();
    }
}

fn gl_del_vert_array(buffer: GLBuffer) {
    unsafe {
        gl::DeleteVertexArrays(1, &buffer);
        gl_check();
    }
}

fn gl_del_frame_buffer(buffer: GLBuffer) {
    unsafe {
        gl::DeleteFramebuffers(1, &buffer);
        gl_check();
    }
}