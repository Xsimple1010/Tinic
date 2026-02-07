use crate::{print_scree::PrintScree, retro_gl::window::GlWindow};
use generics::{
    erro_handle::ErroHandle,
    types::{ArcTMuxte, TMutex},
};
use libretro_sys::binding_libretro::retro_hw_context_type::{
    RETRO_HW_CONTEXT_NONE, RETRO_HW_CONTEXT_OPENGL, RETRO_HW_CONTEXT_OPENGL_CORE,
};
use retro_core::{av_info::AvInfo, RetroVideoEnvCallbacks};
use sdl2::Sdl;
use std::{
    cell::UnsafeCell,
    ffi::{c_uint, c_void},
    path::{Path, PathBuf},
    ptr::null,
    sync::Arc,
};

pub struct RawTextureData {
    pub data: *const c_void,
    pub width: c_uint,
    pub height: c_uint,
    pub pitch: usize,
}

pub trait RetroVideoAPi {
    fn get_window_id(&self) -> u32;

    fn draw_new_frame(&self, texture: &UnsafeCell<RawTextureData>);

    #[doc = "define um novo tamanho para a janela.
        ```
        resize((width, height))
        ```
    "]
    fn resize(&mut self, new_size: (u32, u32));

    fn get_proc_address(&self, proc_name: &str) -> *const ();

    fn enable_full_screen(&mut self);

    fn disable_full_screen(&mut self);

    fn context_destroy(&mut self);

    fn context_reset(&mut self);
}

pub struct RetroVideo {
    window_ctx: ArcTMuxte<Option<Box<dyn RetroVideoAPi>>>,
    texture: ArcTMuxte<UnsafeCell<RawTextureData>>,
}

impl Default for RetroVideo {
    fn default() -> Self {
        Self::new()
    }
}

impl RetroVideo {
    pub fn new() -> Self {
        Self {
            window_ctx: TMutex::new(None),
            texture: TMutex::new(UnsafeCell::new(RawTextureData {
                data: null(),
                pitch: 0,
                height: 0,
                width: 0,
            })),
        }
    }

    //noinspection RsPlaceExpression
    pub fn init(&mut self, sdl: &Sdl, av_info: &Arc<AvInfo>) -> Result<(), ErroHandle> {
        match &av_info.video.graphic_api.context_type {
            RETRO_HW_CONTEXT_OPENGL_CORE | RETRO_HW_CONTEXT_OPENGL | RETRO_HW_CONTEXT_NONE => {
                self.window_ctx
                    .try_load()?
                    .replace(Box::new(GlWindow::new(sdl, av_info)?));
                Ok(())
            }
            // RETRO_HW_CONTEXT_VULKAN => {}
            _ => Err(ErroHandle {
                message: "suporte para a api selecionada não está disponível".to_owned(),
            }),
        }
    }

    pub fn draw_new_frame(&self) -> Result<(), ErroHandle> {
        let mut window = self.window_ctx.try_load()?;

        if let Some(window) = &mut *window {
            let texture = &*self.texture.try_load()?;
            window.draw_new_frame(texture);
        }

        Ok(())
    }

    pub fn get_window_id(&self) -> Result<u32, ErroHandle> {
        let mut window = self.window_ctx.try_load()?;

        if let Some(window) = &mut *window {
            Ok(window.get_window_id())
        } else {
            Ok(0)
        }
    }

    pub fn resize(&self, new_size: (u32, u32)) -> Result<(), ErroHandle> {
        let mut window = self.window_ctx.try_load()?;

        if let Some(window) = &mut *window {
            window.resize(new_size);
        }

        Ok(())
    }

    pub fn print_screen(
        &self,
        out_path: &Path,
        av_info: &Arc<AvInfo>,
    ) -> Result<PathBuf, ErroHandle> {
        PrintScree::take(
            &*self.texture.try_load()?,
            av_info,
            &mut PathBuf::from(out_path),
        )
    }

    pub fn disable_full_screen(&self) -> Result<(), ErroHandle> {
        let mut window = self.window_ctx.try_load()?;

        if let Some(window) = &mut *window {
            window.disable_full_screen();
        }

        Ok(())
    }

    pub fn enable_full_screen(&self) -> Result<(), ErroHandle> {
        let mut window = self.window_ctx.try_load()?;

        if let Some(window) = &mut *window {
            window.enable_full_screen();
        }

        Ok(())
    }

    pub fn get_core_cb(&self) -> RetroVideoCb {
        RetroVideoCb {
            texture: self.texture.clone(),
            window_ctx: self.window_ctx.clone(),
        }
    }
}

pub struct RetroVideoCb {
    pub window_ctx: ArcTMuxte<Option<Box<dyn RetroVideoAPi>>>,
    pub texture: ArcTMuxte<UnsafeCell<RawTextureData>>,
}

impl RetroVideoEnvCallbacks for RetroVideoCb {
    fn video_refresh_callback(
        &self,
        data: *const c_void,
        width: u32,
        height: u32,
        pitch: usize,
    ) -> Result<(), ErroHandle> {
        let mut tex_guard = self.texture.try_load()?;
        let texture = tex_guard.get_mut();

        texture.data = data;
        texture.width = width;
        texture.height = height;
        texture.pitch = pitch;

        Ok(())
    }

    fn get_proc_address(&self, proc_name: &str) -> Result<*const (), ErroHandle> {
        let win_guard = self.window_ctx.try_load()?;

        if let Some(window) = &*win_guard {
            Ok(window.get_proc_address(proc_name))
        } else {
            Ok(null())
        }
    }

    fn context_destroy(&self) -> Result<(), ErroHandle> {
        let mut win_guard = self.window_ctx.try_load()?;

        if let Some(ref mut window) = &mut *win_guard {
            window.context_destroy();
        }

        Ok(())
    }

    fn context_reset(&self) -> Result<(), ErroHandle> {
        let mut win_guard = self.window_ctx.try_load()?;

        if let Some(ref mut window) = &mut *win_guard {
            window.context_reset();
        }

        Ok(())
    }
}
