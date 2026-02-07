use crate::{print_scree::PrintScree, retro_gl::window::RetroGlWindow, SyncData};
use generics::{
    error_handle::ErrorHandle,
    types::{ArcTMutex, TMutex},
};
use libretro_sys::binding_libretro::retro_hw_context_type::{
    RETRO_HW_CONTEXT_NONE, RETRO_HW_CONTEXT_OPENGL, RETRO_HW_CONTEXT_OPENGL_CORE,
};
use retro_core::{
    av_info::{AvInfo, Geometry},
    RetroVideoEnvCallbacks,
};
use std::{
    cell::UnsafeCell,
    ffi::{c_uint, c_void},
    path::{Path, PathBuf},
    ptr::null,
    sync::Arc,
};
use winit::{event_loop::ActiveEventLoop, window::Fullscreen};

pub struct RawTextureData {
    pub data: UnsafeCell<*const c_void>,
    pub width: c_uint,
    pub height: c_uint,
    pub pitch: usize,
}

impl RawTextureData {
    pub fn new() -> Self {
        Self {
            data: UnsafeCell::new(null()),
            pitch: 0,
            height: 0,
            width: 0,
        }
    }
}

pub trait RetroVideoAPi {
    fn request_redraw(&self);

    fn draw_new_frame(&self, texture: &RawTextureData, geo: &Geometry);

    fn get_proc_address(&self, proc_name: &str) -> *const ();

    fn set_full_screen(&mut self, mode: Fullscreen);

    fn context_destroy(&mut self);

    fn context_reset(&mut self);
}

pub struct RetroVideo {
    window_ctx: ArcTMutex<Option<Box<dyn RetroVideoAPi>>>,
    texture: ArcTMutex<RawTextureData>,
    sync_data: ArcTMutex<SyncData>,
    av_info: ArcTMutex<Option<Arc<AvInfo>>>,
}

impl RetroVideo {
    pub fn new(sync_data: ArcTMutex<SyncData>) -> Self {
        Self {
            window_ctx: TMutex::new(None),
            texture: TMutex::new(RawTextureData::new()),
            sync_data,
            av_info: TMutex::new(None),
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
                })
            }
        };

        self.av_info.store(Some(av_info.clone()));

        Ok(())
    }

    pub fn destroy_window(&mut self) {
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

    pub fn get_core_cb(&self) -> RetroVideoCb {
        println!("{:?}", self.av_info);
        RetroVideoCb {
            texture: self.texture.clone(),
            window_ctx: self.window_ctx.clone(),
            sync_data: self.sync_data.clone(),
            av_info: self.av_info.clone(),
        }
    }
}

pub struct RetroVideoCb {
    texture: ArcTMutex<RawTextureData>,
    window_ctx: ArcTMutex<Option<Box<dyn RetroVideoAPi>>>,
    sync_data: ArcTMutex<SyncData>,
    av_info: ArcTMutex<Option<Arc<AvInfo>>>,
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
            if let Some(av_info) = &*self.av_info.load_or(None) {
                win.draw_new_frame(&*texture, &av_info.video.geometry);
            }
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
            win.get_proc_address(proc_name);
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
