use crate::video::RawTextureData;
use generics::{constants::SAVE_IMAGE_EXTENSION_FILE, erro_handle::ErroHandle};
use image::{ImageBuffer, RgbImage};
use libretro_sys::binding_libretro::retro_log_level::RETRO_LOG_ERROR;
use retro_core::core::{retro_pixel_format, AvInfo};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub struct PrintScree {}

impl PrintScree {
    pub fn take(
        raw_texture: &RawTextureData,
        av_info: &Arc<AvInfo>,
        out_path: &mut PathBuf,
        file_name: &str,
    ) -> Result<(), ErroHandle> {
        match &*av_info.video.pixel_format.lock().unwrap() {
            retro_pixel_format::RETRO_PIXEL_FORMAT_XRGB8888 => {
                PrintScree::_from_xrgb8888(raw_texture, out_path, file_name)
            }
            // retro_pixel_format::RETRO_PIXEL_FORMAT_0RGB1555 => ,
            // retro_pixel_format::RETRO_PIXEL_FORMAT_RGB565 => ,
            _ => {
                return Err(ErroHandle {
                    level: RETRO_LOG_ERROR,
                    message: "Formato de pixel desconhecido".to_string(),
                })
            }
        }

        Ok(())
    }

    fn _from_xrgb8888(raw_texture: &RawTextureData, out_path: &mut PathBuf, file_name: &str) {
        let buffer: &[u8] = unsafe {
            std::slice::from_raw_parts(
                raw_texture.data as *const u8,
                (raw_texture.width * raw_texture.height) as usize * 4,
            )
        };

        if buffer.len() != (raw_texture.width * raw_texture.height) as usize * 4 {
            return;
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

        out_path.push(file_name.to_owned() + SAVE_IMAGE_EXTENSION_FILE);

        img.save(Path::new(out_path))
            .map_err(|e| e.to_string())
            .unwrap()
    }
}
