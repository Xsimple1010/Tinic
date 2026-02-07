use crate::raw_texture::RawTextureData;
use crate::retro_env_callback::RetroVideoCb;
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
use winit::{event_loop::ActiveEventLoop, window::Fullscreen};

pub trait RetroWindowsContext {
    fn request_redraw(&self);

    fn draw_new_frame(&self, texture: &RawTextureData);

    fn get_proc_address(&self, proc_name: &str) -> *const ();

    fn set_full_screen(&mut self, mode: Fullscreen);

    fn context_destroy(&mut self);

    fn context_reset(&mut self);

    fn resize(&mut self, width: u32, height: u32);
}

pub struct RetroVideo {
    window_ctx: ArcTMutex<Option<Box<dyn RetroWindowsContext>>>,
    texture: ArcTMutex<RawTextureData>,
    pub sync: RetroSync,
}

impl RetroVideo {
    pub fn new() -> Self {
        Self {
            window_ctx: TMutex::new(None),
            texture: TMutex::new(RawTextureData::new()),
            sync: RetroSync::new(0.0002),
        }
    }

    pub fn init(
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

    pub fn set_full_screen(&mut self, mode: Fullscreen) -> Result<(), ErrorHandle> {
        if let Some(win) = &mut *self.window_ctx.try_load()? {
            win.set_full_screen(mode);
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
