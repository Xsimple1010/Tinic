use crate::raw_texture::RawTextureData;
use image::{ImageBuffer, RgbImage};
use retro_core::{av_info::AvInfo, pixel::PixelFormat};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tinic_generics::error_handle::{ErrorHandle, TinicResult};

pub struct PrintScree;

impl PrintScree {
    pub fn take(
        raw_texture: &RawTextureData,
        av_info: &Arc<AvInfo>,
        out_path: &mut PathBuf,
    ) -> TinicResult<()> {
        if raw_texture.is_hw {
            return Err(ErrorHandle::new("Frame HW não pode ser salvo como imagem"));
        }

        match &*av_info
            .video
            .pixel_format
            .load_or_spawn_err("Pixel está inacessivel")?
        {
            PixelFormat::Xrgb8888 => Self::_from_xrgb8888(raw_texture, out_path),
            PixelFormat::Rgb1555 => Self::_from_0rgb1555(raw_texture, out_path),
            PixelFormat::Rgb565 => Self::_from_rgb565(raw_texture, out_path),
            PixelFormat::Unknown => Err(ErrorHandle::new("Formato de pixel desconhecido")),
        }
    }

    fn _from_xrgb8888(raw_texture: &RawTextureData, out_path: &mut PathBuf) -> TinicResult<()> {
        let width = raw_texture.width as usize;
        let height = raw_texture.height as usize;
        let pitch = raw_texture.pitch;

        let data = &raw_texture.data;

        let mut img_buffer = Vec::with_capacity(width * height * 3);

        for y in 0..height {
            let row_start = y * pitch;
            let row = &data[row_start..row_start + width * 4];

            for pixel in row.chunks_exact(4) {
                let b = pixel[0];
                let g = pixel[1];
                let r = pixel[2];

                img_buffer.extend_from_slice(&[r, g, b]);
            }
        }

        Self::save_image(raw_texture, img_buffer, out_path)
    }

    fn _from_0rgb1555(raw_texture: &RawTextureData, out_path: &mut PathBuf) -> TinicResult<()> {
        let width = raw_texture.width as usize;
        let height = raw_texture.height as usize;
        let pitch = raw_texture.pitch;

        let data = &raw_texture.data;

        let mut img_buffer = Vec::with_capacity(width * height * 3);

        for y in 0..height {
            let row_start = y * pitch;
            let row = &data[row_start..row_start + width * 2];

            for pixel in row.chunks_exact(2) {
                let value = u16::from_le_bytes([pixel[0], pixel[1]]);

                let r5 = ((value >> 10) & 0x1F) as u8;
                let g5 = ((value >> 5) & 0x1F) as u8;
                let b5 = (value & 0x1F) as u8;

                let r = (r5 << 3) | (r5 >> 2);
                let g = (g5 << 3) | (g5 >> 2);
                let b = (b5 << 3) | (b5 >> 2);

                img_buffer.extend_from_slice(&[r, g, b]);
            }
        }

        Self::save_image(raw_texture, img_buffer, out_path)
    }

    fn _from_rgb565(raw_texture: &RawTextureData, out_path: &mut PathBuf) -> TinicResult<()> {
        let width = raw_texture.width as usize;
        let height = raw_texture.height as usize;
        let pitch = raw_texture.pitch;

        let data = &raw_texture.data;

        let mut img_buffer = Vec::with_capacity(width * height * 3);

        for y in 0..height {
            let row_start = y * pitch;
            let row = &data[row_start..row_start + width * 2];

            for pixel in row.as_chunks::<2>().0 {
                let value = u16::from_le_bytes([pixel[0], pixel[1]]);

                let r5 = ((value >> 11) & 0x1F) as u8;
                let g6 = ((value >> 5) & 0x3F) as u8;
                let b5 = (value & 0x1F) as u8;

                let r = (r5 << 3) | (r5 >> 2);
                let g = (g6 << 2) | (g6 >> 4);
                let b = (b5 << 3) | (b5 >> 2);

                img_buffer.extend_from_slice(&[r, g, b]);
            }
        }

        Self::save_image(raw_texture, img_buffer, out_path)
    }

    fn save_image(
        raw_texture: &RawTextureData,
        buffer: Vec<u8>,
        out_path: &PathBuf,
    ) -> TinicResult<()> {
        let img: RgbImage = ImageBuffer::from_raw(raw_texture.width, raw_texture.height, buffer)
            .ok_or_else(|| ErrorHandle::new("Falha ao criar ImageBuffer"))?;

        img.save(Path::new(out_path))
            .map_err(|e| ErrorHandle::new(&e.to_string()))?;

        Ok(())
    }
}
