use crate::*;

/*****************************************************************************
*                               STRUCTS
******************************************************************************/

pub type GLTextureBuffer = gl::types::GLuint;
type GLTextureType = gl::types::GLenum;

/*****************************************************************************
*                               HELPERS
******************************************************************************/

pub fn gl_gen_texture() -> GLTextureBuffer {
    unsafe {
        let mut buffer: GLTextureBuffer = 0;
        gl::GenTextures(1, &mut buffer as *mut GLTextureBuffer);
        gl_check();
        buffer
    }
}

pub fn gl_del_texture(texture: GLTextureBuffer) {
    unsafe {
        gl::DeleteTextures(1, &texture as *const u32);
        gl_check();
    }
}

pub fn gl_bind_texture(target: GLTextureType, texture: GLTextureBuffer) {
    unsafe {
        gl::BindTexture(target, texture);
        gl_check();
    }
}

pub fn gl_unbind_texture(target: GLTextureType) {
    unsafe {
        gl::BindTexture(target, 0);
        gl_check();
    }
}

pub fn gl_tex_parami(target: GLTextureType, name: GLenum, param: u32) {
    unsafe {
        gl::TexParameteri(target, name, param as i32);
        gl_check();
    }
}

pub fn gl_gen_mips(target: GLTextureType) {
    unsafe {
        gl::GenerateMipmap(target);
        gl_check();
    }
}

pub fn gl_tex_image_2d(internal_format: u32, width: i32, height: i32, format: u32, data: *const c_void) {
    unsafe {
        gl::TexImage2D(gl::TEXTURE_2D, 0, internal_format as i32, width, height, 0, format, gl::UNSIGNED_BYTE, data);
        gl_check();
    }
}

pub fn gl_active_texture(slot: u32) {
    unsafe {
        gl::ActiveTexture(gl::TEXTURE0 + slot);
        gl_check();
    }
}