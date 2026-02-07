use crate::video::RawTextureData;
use generics::error_handle::ErrorHandle;
use image::{ImageBuffer, RgbImage};
use libretro_sys::binding_libretro::retro_pixel_format;
use retro_core::av_info::AvInfo;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub struct PrintScree;

impl PrintScree {
    pub fn take(
        raw_texture: &RawTextureData,
        av_info: &Arc<AvInfo>,
        out_path: &mut PathBuf,
    ) -> Result<(), ErrorHandle> {
        match &*av_info.video.pixel_format.read()? {
            retro_pixel_format::RETRO_PIXEL_FORMAT_XRGB8888 => {
                PrintScree::_from_xrgb8888(raw_texture, out_path)
            }
            // retro_pixel_format::RETRO_PIXEL_FORMAT_0RGB1555 => ,
            // retro_pixel_format::RETRO_PIXEL_FORMAT_RGB565 => ,
            _ => Err(ErrorHandle {
                message: "Formato de pixel desconhecido".to_string(),
            }),
        }
    }

    fn _from_xrgb8888(
        raw_texture: &RawTextureData,
        out_path: &mut PathBuf,
    ) -> Result<(), ErrorHandle> {
        let data = unsafe { raw_texture.data.get().read() };

        let buffer: &[u8] = unsafe {
            std::slice::from_raw_parts(
                data as *const u8,
                (raw_texture.width * raw_texture.height) as usize * 4,
            )
        };

        if buffer.len() != (raw_texture.width * raw_texture.height) as usize * 4 {
            return Err(ErrorHandle {
                message: "Tamanho do buffer video esta errado".to_string(),
            });
        }

        // Crie um buffer de imagem a partir do buffer de textura, ignorando o componente X
        let mut img_buffer =
            Vec::with_capacity((raw_texture.width * raw_texture.height * 3) as usize);

        for chunk in buffer.chunks(4) {
            img_buffer.push(chunk[2]); // R
            img_buffer.push(chunk[1]); // G
            img_buffer.push(chunk[0]); // B
        }

        let img: RgbImage =
            ImageBuffer::from_raw(raw_texture.width, raw_texture.height, img_buffer).unwrap();

        img.save(Path::new(out_path))
            .map_err(|e| e.to_string())
            .unwrap();

        Ok(())
    }
}
