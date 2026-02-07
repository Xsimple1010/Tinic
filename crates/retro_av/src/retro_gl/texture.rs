use super::{gl::gl, pixel::Pixel};
use crate::video::RawTextureData;
use generics::error_handle::ErrorHandle;
use gl::types::GLuint;
use retro_core::av_info::AvInfo;
use std::{
    ptr::null,
    rc::Rc,
    sync::{atomic::Ordering, Arc},
};

pub type TexturePosition = [f32; 2];

pub struct Texture2D {
    id: GLuint,
    pixel: Pixel,
    gl: Rc<gl::Gl>,
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteTextures(1, &self.id) }
    }
}

impl Texture2D {
    pub fn active(&self) {
        unsafe {
            self.gl.ActiveTexture(gl::TEXTURE0);
            self.gl.BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn push(&self, texture: &RawTextureData) {
        let param = texture.pitch as i32 / self.pixel.bpm;

        unsafe {
            self.gl.BindTexture(gl::TEXTURE0, self.id);
            self.gl.PixelStorei(gl::UNPACK_ROW_LENGTH, param);
            self.gl.TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                0,
                0,
                texture.width as i32,
                texture.height as i32,
                self.pixel.typ,
                self.pixel.format,
                texture.data.get().read(),
            );
            self.gl.BindTexture(gl::TEXTURE0, 0);
        }
    }

    pub fn get_id(&self) -> GLuint {
        self.id
    }

    pub fn new(av_info: &Arc<AvInfo>, gl: Rc<gl::Gl>) -> Result<Texture2D, ErrorHandle> {
        let mut id = 0;
        let geo = &av_info.video.geometry;
        let pixel = Pixel::new(&*av_info.video.pixel_format.try_load()?)?;

        unsafe {
            gl.GenTextures(1, &mut id);
            gl.BindTexture(gl::TEXTURE_2D, id);

            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl.TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB8 as i32,
                geo.max_width.load(Ordering::SeqCst) as i32,
                geo.max_height.load(Ordering::SeqCst) as i32,
                0,
                pixel.typ,
                pixel.format,
                null(),
            );

            gl.BindTexture(gl::TEXTURE_2D, 0);

            Ok(Texture2D { id, pixel, gl })
        }
    }
}
