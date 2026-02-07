use crate::raw_texture::RawTextureData;
use crate::retro_window::RetroWindowContext;
use generics::error_handle::ErrorHandle;
use generics::types::ArcTMutex;
use retro_core::RetroVideoEnvCallbacks;
use std::ffi::c_void;
use std::ptr::null;

pub struct RetroVideoCb {
    texture: ArcTMutex<RawTextureData>,
    window_ctx: ArcTMutex<Option<Box<dyn RetroWindowContext>>>,
}

impl RetroVideoCb {
    pub fn new(
        texture: ArcTMutex<RawTextureData>,
        window_ctx: ArcTMutex<Option<Box<dyn RetroWindowContext>>>,
    ) -> Self {
        Self {
            texture,
            window_ctx,
        }
    }
}

impl RetroVideoEnvCallbacks for RetroVideoCb {
    fn video_refresh_callback(
        &self,
        data: *const c_void,
        width: u32,
        height: u32,
        pitch: usize,
    ) -> Result<(), ErrorHandle> {
        let mut texture = self.texture.try_load()?;
        {
            let tex_data = texture.data.get_mut();

            *tex_data = data;
            texture.width = width;
            texture.height = height;
            texture.pitch = pitch;
        }

        if let Some(win) = &mut *self.window_ctx.try_load()? {
            win.draw_new_frame(&texture);
        }

        Ok(())
    }

    fn context_reset(&self) -> Result<(), ErrorHandle> {
        if let Some(win) = &mut *self.window_ctx.try_load()? {
            win.context_reset();
        }
        Ok(())
    }

    fn get_proc_address(&self, proc_name: &str) -> Result<*const (), ErrorHandle> {
        if let Some(win) = &mut *self.window_ctx.try_load()? {
            return Ok(win.get_proc_address(proc_name));
        }

        Ok(null())
    }

    fn context_destroy(&self) -> Result<(), ErrorHandle> {
        if let Some(win) = &mut *self.window_ctx.try_load()? {
            win.context_destroy();
        }
        Ok(())
    }
}
