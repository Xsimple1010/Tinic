use super::gl::gl::{
    self,
    types::{GLenum, GLuint},
};
use generics::error_handle::ErrorHandle;
use gl::COMPILE_STATUS;
use std::{ffi::CString, ptr::null, rc::Rc};

pub struct Shader {
    pub id: GLuint,
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
    ) -> Result<Shader, ErrorHandle> {
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
                        let log = CString::new("")?;
                        let log_ptr = log.into_raw();
                        let mut length = 0;

                        gl.GetShaderInfoLog(id, 4096, &mut length, log_ptr);

                        let log = CString::from_raw(log_ptr);

                        return Err(ErrorHandle {
                            message: log.into_string().unwrap(),
                        });
                    }

                    Ok(Self { id, gl })
                }
                Err(e) => Err(ErrorHandle {
                    message: "Erro ao tentar criar um shader: ".to_string()
                        + e.to_string().as_str(),
                }),
            }
        }
    }
}
