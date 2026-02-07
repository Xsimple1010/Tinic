use std::rc::Rc;

use super::gl::gl::{
    types::{GLenum, GLsizei, GLuint},
    Gl, RENDERBUFFER,
};

pub struct RenderBuffer {
    id: GLuint,
    gl: Rc<Gl>,
}

impl Drop for RenderBuffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteRenderbuffers(1, [self.id].as_ptr());
        }
    }
}

impl RenderBuffer {
    pub fn new(gl: Rc<Gl>) -> Self {
        let mut id = 0;

        unsafe {
            gl.GenRenderbuffers(1, &mut id);
        }

        Self { id, gl }
    }

    pub fn get_id(&self) -> GLuint {
        self.id
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindRenderbuffer(RENDERBUFFER, self.id);
        }
    }

    pub fn un_bind(&self) {
        unsafe {
            self.gl.BindRenderbuffer(RENDERBUFFER, 0);
        }
    }

    pub fn storage(&self, internal_format: GLenum, width: GLsizei, height: GLsizei) {
        unsafe {
            self.gl
                .RenderbufferStorage(RENDERBUFFER, internal_format, width, height)
        }
    }
}
