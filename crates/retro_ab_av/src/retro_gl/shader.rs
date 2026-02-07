use gl::COMPILE_STATUS;
use retro_ab::{erro_handle::ErroHandle, retro_sys::retro_log_level};
use std::{ffi::CString, ptr::null, rc::Rc};

use super::gl::gl::{
    self,
    types::{GLenum, GLuint},
};

pub struct Shader {
    // pub program: GLuint,
    // pub vao: GLuint,
    // pub vbo: GLuint,
    // pub i_pos: GLint,
    // pub i_text_pos: GLint,
    pub id: GLuint,
    // pub u_tex: GLint,
    // _u_mvp: GLint,
    gl: Rc<gl::Gl>,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}

impl Shader {
    pub fn new(
        shader_type: GLenum,
        source_code: &str,
        gl: Rc<gl::Gl>,
    ) -> Result<Shader, ErroHandle> {
        unsafe {
            let id = gl.CreateShader(shader_type);

            let source = CString::new(source_code);

            match source {
                Ok(source) => {
                    let source = source.as_c_str().as_ptr();

                    gl.ShaderSource(id, 1, &source, null());
                    gl.CompileShader(id);

                    let mut status = 0;
                    gl.GetShaderiv(id, COMPILE_STATUS, &mut status);

                    if status == 0 {
                        let log = CString::new("").unwrap();
                        let log_ptr = log.into_raw();
                        let mut length = 0;

                        gl.GetShaderInfoLog(id, 4096, &mut length, log_ptr);

                        let log = CString::from_raw(log_ptr);

                        return Err(ErroHandle {
                            level: retro_log_level::RETRO_LOG_ERROR,
                            message: log.into_string().unwrap(),
                        });
                    }

                    Ok(Self { id, gl })
                }
                Err(e) => Err(ErroHandle {
                    level: retro_log_level::RETRO_LOG_ERROR,
                    message: "Erro ao tentar criar um shader: ".to_string()
                        + e.to_string().as_str(),
                }),
            }
        }
    }
}
