use super::{
    gl::gl::{self, types::GLuint},
    shader::Shader,
};
use crate::retro_gl::gl::gl::types::GLint;
use generics::erro_handle::ErroHandle;
use libretro_sys::binding_libretro::retro_log_level::RETRO_LOG_ERROR;
use std::{ffi::CString, rc::Rc};

pub struct ShaderProgram {
    id: GLuint,
    gl: Rc<gl::Gl>,
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}

impl ShaderProgram {
    pub fn new(shaders: &[Shader], gl: Rc<gl::Gl>) -> Result<ShaderProgram, ErroHandle> {
        unsafe {
            let id = gl.CreateProgram();

            for shader in shaders {
                gl.AttachShader(id, shader.id);
            }

            gl.LinkProgram(id);
            gl.ValidateProgram(id);

            let mut status = 0;

            gl.GetProgramiv(id, gl::LINK_STATUS, &mut status);

            if status == 0 {
                let mut error_log_size: GLint = 0;
                gl.GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut error_log_size);
                let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
                gl.GetProgramInfoLog(
                    id,
                    error_log_size,
                    &mut error_log_size,
                    error_log.as_mut_ptr() as *mut _,
                );

                error_log.set_len(error_log_size as usize);
                let log = String::from_utf8(error_log);

                return match log {
                    Ok(log) => Err(ErroHandle {
                        level: RETRO_LOG_ERROR,
                        message: log,
                    }),
                    Err(e) => Err(ErroHandle {
                        level: RETRO_LOG_ERROR,
                        message: e.to_string(),
                    }),
                };
            }

            Ok(Self { id, gl })
        }
    }

    pub fn get_attribute(&self, name: &str) -> GLint {
        let param_name = CString::new(name).unwrap();
        unsafe { self.gl.GetAttribLocation(self.id, param_name.as_ptr()) }
    }

    pub fn get_uniform(&self, name: &str) -> GLint {
        let param_name = CString::new(name).unwrap();
        unsafe { self.gl.GetUniformLocation(self.id, param_name.as_ptr()) }
    }

    pub fn use_program(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }

    pub fn un_use_program(&self) {
        unsafe {
            self.gl.UseProgram(0);
        }
    }
}
