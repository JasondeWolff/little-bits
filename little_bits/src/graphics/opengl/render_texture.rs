extern crate gl_wrapper;
pub use gl_wrapper::*;

pub struct GLRenderTexture {
    tex: GLTexture
}

impl GLRenderTexture {
    pub fn new(width: usize, height: usize) -> Self {
        let tex = GLTexture::new(gl::TEXTURE_2D);

        tex.bind(); {
            gl_tex_image_2df(gl::RGBA16F, width as i32, height as i32, gl::RGBA, std::ptr::null());
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR);
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR);
        } tex.unbind();

        GLRenderTexture {
            tex: tex
        }
    }
    pub fn bind(&self, slot: u32) {
        gl_active_texture(slot);
        self.tex.bind();
    }

    pub fn unbind(&self) {
        self.tex.unbind();
    }

    pub fn tex(&self) -> &GLTexture {
        &self.tex
    }
}