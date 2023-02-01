extern crate gl_wrapper;
pub use gl_wrapper::*;

use crate::graphics::opengl::render_texture::GLRenderTexture;
use crate::Shared;

use std::collections::HashMap;

pub struct GLRenderTarget {
    fbo: GLFBO,
    rbo: GLRBO,
    textures: HashMap<GLRenderAttachment, GLRenderTexture>
}

#[derive( std::cmp::Eq, std::cmp::PartialEq, Hash)]
pub enum GLRenderAttachment {
    Color(u32),
    Depth
}

impl GLRenderAttachment {
    pub fn to_gl(&self) -> u32 {
        match self {
            GLRenderAttachment::Color(slot) => {
                unsafe {
                    assert!(*slot < 32, "Failed to create GLRenderTexture. (Max color slot is 31)");
                    let slot0: u32 = std::mem::transmute(gl::COLOR_ATTACHMENT0);
                    std::mem::transmute(slot0 + slot)
                }
            },
            GLRenderAttachment::Depth => gl::DEPTH_ATTACHMENT
        }
    }
}

impl GLRenderTarget {
    pub fn new(width: usize, height: usize) -> Self {
        let fbo = GLFBO::new();
        let rbo = GLRBO::new();
        let textures = HashMap::new();

        fbo.bind(); {
            rbo.bind(); {
                gl_render_buffer_storage(gl::DEPTH_COMPONENT, width as i32, height as i32);
            } rbo.unbind();

            gl_frame_buffer_render_buffer(&rbo, gl::DEPTH_ATTACHMENT);

            fbo.check_status();
        } fbo.unbind();

        GLRenderTarget {
            fbo: fbo,
            rbo: rbo,
            textures: textures
        }
    }

    pub fn set_texture(&mut self, attachment: GLRenderAttachment, texture: GLRenderTexture) {
        self.bind(); {
            let glattachment = attachment.to_gl();
            gl_frame_buffer_texture_2d(&texture.tex(), glattachment);
        } self.unbind();

        match self.textures.get_mut(&attachment) {
            Some(old_texture) => {
                *old_texture = texture;
            },
            None => {
                self.textures.insert(attachment, texture);
            }
        }

        //self.set_active_buffers();
    }

    fn set_active_buffers(&self) {
        let mut attachments = Vec::new();
        for (attachment, _) in &self.textures {
            attachments.push(attachment.to_gl());
        }

        self.bind(); {
            gl_draw_buffers(attachments.len(), attachments.as_ptr() as *const GLenum);
        } self.unbind();
    }

    pub fn get_texture(&mut self, attachment: GLRenderAttachment) -> Option<&mut GLRenderTexture> {
        match self.textures.get_mut(&attachment) {
            Some(texture) => Some(texture),
            None => None
        }
    }

    pub fn bind(&self) {
        self.fbo.bind();
    }

    pub fn unbind(&self) {
        self.fbo.unbind();
    }

    pub fn check(&self) {
        self.fbo.check_status();
    }
}