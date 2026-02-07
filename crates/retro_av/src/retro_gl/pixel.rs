use super::gl::gl::{self, types::GLuint};
use generics::erro_handle::ErroHandle;
use libretro_sys::binding_libretro::retro_pixel_format;
use std::mem::size_of;

pub struct Pixel {
    pub format: GLuint,
    pub typ: GLuint,
    pub bpm: i32,
}

impl Pixel {
    pub fn new(retro_pixel: &retro_pixel_format) -> Result<Pixel, ErroHandle> {
        match retro_pixel {
            retro_pixel_format::RETRO_PIXEL_FORMAT_XRGB8888 => Ok(Pixel {
                format: gl::UNSIGNED_INT_8_8_8_8_REV,
                typ: gl::BGRA,
                bpm: size_of::<u32>() as i32,
            }),
            retro_pixel_format::RETRO_PIXEL_FORMAT_0RGB1555 => Ok(Pixel {
                format: gl::UNSIGNED_SHORT_5_5_5_1,
                typ: gl::BGRA,
                bpm: size_of::<u16>() as i32,
            }),
            retro_pixel_format::RETRO_PIXEL_FORMAT_RGB565 => Ok(Pixel {
                format: gl::UNSIGNED_SHORT_5_6_5,
                typ: gl::RGB,
                bpm: size_of::<u16>() as i32,
            }),
            _ => Err(ErroHandle {
                message: "Formato de pixel desconhecido".to_string(),
            }),
        }
    }
}
