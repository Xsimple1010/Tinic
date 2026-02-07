use crate::raw_texture::RawTextureData;
use crate::retro_env_callback::RetroVideoCb;
use crate::retro_window::{RetroWindowContext, RetroWindowMode};
use crate::sync::RetroSync;
use crate::{print_scree::PrintScree, retro_gl::window::RetroGlWindow};
use generics::{
    error_handle::ErrorHandle,
    types::{ArcTMutex, TMutex},
};
use libretro_sys::binding_libretro::retro_hw_context_type::{
    RETRO_HW_CONTEXT_NONE, RETRO_HW_CONTEXT_OPENGL, RETRO_HW_CONTEXT_OPENGL_CORE,
};
use retro_core::av_info::AvInfo;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use winit::event_loop::ActiveEventLoop;

pub struct RetroVideo {
    window_ctx: ArcTMutex<Option<Box<dyn RetroWindowContext>>>,
    texture: ArcTMutex<RawTextureData>,
    pub sync: RetroSync,
}

impl Default for RetroVideo {
    fn default() -> Self {
        Self {
            window_ctx: TMutex::new(None),
            texture: TMutex::new(RawTextureData::new()),
            sync: RetroSync::new(0.0002),
        }
    }
}

impl RetroVideo {
    pub fn create_window(
        &mut self,
        av_info: &Arc<AvInfo>,
        event_loop: &ActiveEventLoop,
    ) -> Result<(), ErrorHandle> {
        match &av_info.video.graphic_api.context_type {
            RETRO_HW_CONTEXT_OPENGL_CORE | RETRO_HW_CONTEXT_OPENGL | RETRO_HW_CONTEXT_NONE => {
                self.window_ctx
                    .try_load()?
                    .replace(Box::new(RetroGlWindow::new(event_loop, av_info)));
            }
            // RETRO_HW_CONTEXT_VULKAN => {}
            _ => {
                return Err(ErrorHandle {
                    message: "suporte para a api selecionada não está disponível".to_owned(),
                });
            }
        };

        Ok(())
    }

    pub fn create_draw_context(&self) -> Result<(), ErrorHandle> {
        let window_ctx = &mut *self.window_ctx.try_load()?;

        let window_ctx = match window_ctx {
            Some(ctx) => ctx,
            None => return Err(ErrorHandle::new("windows context is not initialized")),
        };

        window_ctx.context_reset();
        Ok(())
    }

    pub fn draw_context_as_initialized(&self) -> bool {
        let window_ctx = match self.window_ctx.try_load() {
            Ok(ctx) => ctx,
            Err(_) => return false,
        };

        match &*window_ctx {
            Some(ctx) => ctx.draw_context_as_initialized(),
            None => false,
        }
    }

    pub fn destroy_window(&self) {
        self.window_ctx.store(None);
        self.texture.store(RawTextureData::new());
    }

    pub fn request_redraw(&self) -> Result<(), ErrorHandle> {
        if let Some(win) = &*self.window_ctx.try_load()? {
            win.request_redraw();
        }

        Ok(())
    }

    pub fn print_screen(&self, out_path: &Path, av_info: &Arc<AvInfo>) -> Result<(), ErrorHandle> {
        PrintScree::take(
            &*self.texture.try_load()?,
            av_info,
            &mut PathBuf::from(out_path),
        )
    }

    pub fn toggle_window_mode(&mut self) -> Result<(), ErrorHandle> {
        if let Some(win) = &mut *self.window_ctx.try_load()? {
            win.toggle_window_model();
        }
        Ok(())
    }

    pub fn set_window_mode(&mut self, mode: RetroWindowMode) -> Result<(), ErrorHandle> {
        if let Some(win) = &mut *self.window_ctx.try_load()? {
            win.set_window_mode(mode);
        }
        Ok(())
    }

    pub fn resize_window(&mut self, width: u32, height: u32) -> Result<(), ErrorHandle> {
        if let Some(win) = &mut *self.window_ctx.try_load()? {
            win.resize(width, height);
        }

        Ok(())
    }

    pub fn get_core_cb(&self) -> RetroVideoCb {
        RetroVideoCb::new(self.texture.clone(), self.window_ctx.clone())
    }
}
