use crate::video::RawTextureData;
use generics::error_handle::ErrorHandle;
use image::{ImageBuffer,  RgbImage};
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
        match &*av_info.video.pixel_format.load_or_spaw_err("Pixel está inacessivel")? {
            retro_pixel_format::RETRO_PIXEL_FORMAT_XRGB8888 => {
                Self::_from_xrgb8888(raw_texture, out_path)
            }
            retro_pixel_format::RETRO_PIXEL_FORMAT_0RGB1555 => Self::_from_0rgb1555(raw_texture, out_path),
            retro_pixel_format::RETRO_PIXEL_FORMAT_RGB565 => Self::_from_rgb565(raw_texture, out_path),
            _ => Err(ErrorHandle::new("Formato de pixel desconhecido")),
        }
    }

    fn _from_xrgb8888(
        raw_texture: &RawTextureData,
        out_path: &mut PathBuf,
    ) -> Result<(), ErrorHandle> {
        let data_ptr = unsafe { raw_texture.data.get().read() as *const u8 };

        let width = raw_texture.width as usize;
        let height = raw_texture.height as usize;
        let pitch = raw_texture.pitch;

        let mut img_buffer =
            Vec::with_capacity(width * height * 3);

        for y in 0..height {
            let row_ptr =  unsafe {data_ptr.add(y * pitch)};
            let rows: &[u8] = unsafe {
                std::slice::from_raw_parts(row_ptr, width * 4)
            };


            for pixel in rows.chunks_exact(4) {
                let b = pixel[0];
                let g = pixel[1];
                let r = pixel[2];
                // pixel[3] = X (ignorar)

                img_buffer.push(r);
                img_buffer.push(g);
                img_buffer.push(b);
            }
        }


        let img: RgbImage =
            ImageBuffer::from_raw(raw_texture.width, raw_texture.height, img_buffer)
                .ok_or_else(|| ErrorHandle::new("Falha ao criar ImageBuffer"))?;

        img.save(Path::new(out_path))
            .map_err(|e| e.to_string())
            .unwrap();

        Ok(())
    }

    fn _from_0rgb1555(
        raw_texture: &RawTextureData,
        out_path: &mut PathBuf,
    ) -> Result<(), ErrorHandle> {
        let data_ptr = unsafe { raw_texture.data.get().read() as *const u8 };

        let mut img_buffer =
            Vec::with_capacity((raw_texture.width * raw_texture.height * 3) as usize);

        let width = raw_texture.width as usize;
        let height = raw_texture.height as usize;
        let pitch = raw_texture.pitch;

        for y in 0..height {
            let row_ptr = unsafe { data_ptr.add(y * pitch) } as *const u16;
            let row: &[u16] = unsafe {
                std::slice::from_raw_parts(row_ptr, width)
            };

            for &pixel in row {
                // 0RGB1555
                let r5 = ((pixel >> 10) & 0x1F) as u8;
                let g5 = ((pixel >> 5) & 0x1F) as u8;
                let b5 = (pixel & 0x1F) as u8;

                // 5 bits → 8 bits
                let r = (r5 << 3) | (r5 >> 2);
                let g = (g5 << 3) | (g5 >> 2);
                let b = (b5 << 3) | (b5 >> 2);

                img_buffer.push(r);
                img_buffer.push(g);
                img_buffer.push(b);
            }
        }

        let img: RgbImage =
            ImageBuffer::from_raw(raw_texture.width, raw_texture.height, img_buffer)
                .ok_or_else(|| ErrorHandle::new("Falha ao criar ImageBuffer"))?;

        img.save(Path::new(out_path))
            .map_err(|e| ErrorHandle::new(&e.to_string()))?;

        Ok(())
    }

    fn _from_rgb565(
        raw_texture: &RawTextureData,
        out_path: &mut PathBuf,
    ) -> Result<(), ErrorHandle> {
        let data_ptr = unsafe { raw_texture.data.get().read() as *const u8 };

        let width = raw_texture.width as usize;
        let height = raw_texture.height as usize;
        let pitch = raw_texture.pitch; // 🔴 ISSO É O QUE FALTAVA

        let mut img_buffer = Vec::with_capacity(width * height * 3);

        for y in 0..height {
            let row_ptr = unsafe { data_ptr.add(y * pitch) } as *const u16;
            let row: &[u16] = unsafe {
                std::slice::from_raw_parts(row_ptr, width)
            };

            for &pixel in row {
                let r5 = ((pixel >> 11) & 0x1F) as u8;
                let g6 = ((pixel >> 5) & 0x3F) as u8;
                let b5 = (pixel & 0x1F) as u8;

                let r = (r5 << 3) | (r5 >> 2);
                let g = (g6 << 2) | (g6 >> 4);
                let b = (b5 << 3) | (b5 >> 2);

                img_buffer.push(r);
                img_buffer.push(g);
                img_buffer.push(b);
            }
        }

        let img: RgbImage =
            ImageBuffer::from_raw(raw_texture.width, raw_texture.height, img_buffer)
                .ok_or_else(|| ErrorHandle::new("Falha ao criar ImageBuffer"))?;

        img.save(Path::new(out_path))
            .map_err(|e| ErrorHandle::new(&e.to_string()))?;

        Ok(())
    }

}
