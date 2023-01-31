extern crate gl_wrapper;
pub use gl_wrapper::*;

pub struct GLRenderTexture {
    fbo: GLFBO,
    rbo: GLRBO,
    tex: GLTexture
}

pub enum GLRenderAttachment {
    Color(u32),
    Depth
}

impl GLRenderTexture {
    pub fn new(width: usize, height: usize, attachment: GLRenderAttachment) -> Self {
        let fbo = GLFBO::new();
        let rbo = GLRBO::new();
        let tex = GLTexture::new(gl::TEXTURE_2D);

        fbo.bind(); {
            tex.bind(); {
                gl_tex_image_2d(gl::RGB, width as i32, height as i32, gl::RGB, std::ptr::null());
                gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR);
                gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR);
            } tex.unbind();

            let attachment = match attachment {
                GLRenderAttachment::Color(slot) => {
                    unsafe {
                        assert!(slot < 32, "Failed to create GLRenderTexture. (Max color slot is 31)");
                        let slot0: u32 = std::mem::transmute(gl::COLOR_ATTACHMENT0);
                        std::mem::transmute(slot0 + slot)
                    }
                },
                GLRenderAttachment::Depth => gl::DEPTH_ATTACHMENT
            };
            gl_frame_buffer_texture_2d(&tex, attachment);

            rbo.bind(); {
                gl_render_buffer_storage(gl::DEPTH24_STENCIL8, width as i32, height as i32);
            } rbo.unbind();

            gl_frame_buffer_render_buffer(&rbo, gl::DEPTH_STENCIL_ATTACHMENT);

            fbo.check_status();
        } fbo.unbind();

        GLRenderTexture {
            fbo: fbo,
            rbo: rbo,
            tex: tex
        }
    }

    pub fn bind(&self) {
        self.fbo.bind();
    }

    pub fn unbind(&self) {
        self.fbo.unbind();
    }

    pub fn tex(&self) -> &GLTexture {
        &self.tex
    }
}