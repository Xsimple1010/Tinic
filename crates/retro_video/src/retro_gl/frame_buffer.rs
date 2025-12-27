use std::rc::Rc;

use super::{
    gl::gl::{
        types::{GLenum, GLuint},
        Gl, COLOR_ATTACHMENT0, FRAMEBUFFER, RENDERBUFFER, TEXTURE_2D,
    },
    texture::Texture2D,
};

pub struct FrameBuffer {
    id: GLuint,
    gl: Rc<Gl>,
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteFramebuffers(1, [self.id].as_ptr());
        }
    }
}

impl FrameBuffer {
    pub fn new(gl: Rc<Gl>) -> Self {
        let mut id = 0;

        unsafe {
            gl.GenFramebuffers(1, &mut id);
        }

        Self { id, gl }
    }

    pub fn get_id(&self) -> GLuint {
        self.id
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindFramebuffer(FRAMEBUFFER, self.id);
        }
    }

    pub fn un_bind(&self) {
        unsafe {
            self.gl.BindFramebuffer(FRAMEBUFFER, 0);
        }
    }

    pub fn attach_texture(&self, texture: &Texture2D) {
        unsafe {
            self.gl.FramebufferTexture2D(
                FRAMEBUFFER,
                COLOR_ATTACHMENT0,
                TEXTURE_2D,
                texture.get_id(),
                0,
            );
        }
    }

    pub fn attach_render_buffer(&self, attachment: GLenum, rbo_id: GLuint) {
        unsafe {
            self.gl
                .FramebufferRenderbuffer(FRAMEBUFFER, attachment, RENDERBUFFER, rbo_id);
        }
    }
}
