use std::{mem::size_of, ptr::null, rc::Rc};

use super::gl::gl::{self, types::*};

pub struct VertexArray {
    id: GLuint,
    gl: Rc<gl::Gl>,
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteVertexArrays(1, [self.id].as_ptr());
        }
    }
}

impl VertexArray {
    pub fn new(gl: Rc<gl::Gl>) -> VertexArray {
        let mut id = 0;

        unsafe {
            gl.GenVertexArrays(1, &mut id);
        }

        Self { id, gl }
    }

    pub fn set_attribute<V: Sized>(&self, atribute_pos: GLuint, components: GLint, offset: GLint) {
        unsafe {
            self.bind();
            self.gl.VertexAttribPointer(
                atribute_pos,
                components,
                gl::FLOAT,
                gl::FALSE,
                size_of::<V>().try_into().unwrap(),
                if offset == 0 {
                    null()
                } else {
                    offset as *const _
                },
            );
            self.gl.EnableVertexAttribArray(atribute_pos);
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindVertexArray(self.id);
        }
    }

    pub fn un_bind(&self) {
        unsafe {
            self.gl.BindVertexArray(0);
        }
    }
}
