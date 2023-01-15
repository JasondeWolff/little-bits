#![allow(dead_code)]

#[path = "helpers.rs"] mod helpers;
use helpers::*;

use crate::resources::Image;

struct GLTexture2D {
    buffer: GLTextureBuffer
}

impl GLTexture2D {
    pub fn new(image: &Image) -> Self {
        let texture = GLTexture2D {
            buffer: gl_gen_texture()
        };

        texture.bind_slotless(); {
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT);
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT);
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR);
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR);

            let format = match image.channel_count {
                1 => gl::RED,
                3 => gl::RGB,
                _ => gl::RGBA
            };

            gl_tex_image_2d(gl::RGBA, image.dimensions.x, image.dimensions.y, format, image.data as *const c_void);
            gl_gen_mips(texture.buffer);
        } texture.unbind();

        texture
    }

    pub fn bind(&self, slot: u32) {
        gl_active_texture(slot);
        gl_bind_texture(gl::TEXTURE_2D, self.buffer);
    }

    pub fn bind_slotless(&self) {
        gl_bind_texture(gl::TEXTURE_2D, self.buffer);
    }

    pub fn unbind(&self) {
        gl_unbind_texture(gl::TEXTURE_2D);
    }
}

impl Drop for GLTexture2D {
    fn drop(&mut self) {
        gl_del_texture(self.buffer);
    }
}