use crate::{print_scree::PrintScree, retro_gl::window::GlWindow};
use generics::erro_handle::ErroHandle;
use libretro_sys::binding_libretro::{
    retro_hw_context_type::{
        RETRO_HW_CONTEXT_NONE, RETRO_HW_CONTEXT_OPENGL, RETRO_HW_CONTEXT_OPENGL_CORE,
    },
    retro_log_level,
};
use retro_core::{av_info::AvInfo, RetroVideoEnvCallbacks};
use sdl2::Sdl;
use std::{
    ffi::{c_uint, c_void},
    path::{Path, PathBuf},
    ptr::{addr_of, addr_of_mut, null},
    sync::Arc,
};

static mut WINDOW_CTX: Option<Box<dyn RetroVideoAPi>> = None;

pub struct RawTextureData {
    pub data: *const c_void,
    pub width: c_uint,
    pub height: c_uint,
    pub pitch: usize,
}

static mut RAW_TEX_POINTER: RawTextureData = RawTextureData {
    data: null(),
    pitch: 0,
    height: 0,
    width: 0,
};

pub trait RetroVideoAPi {
    fn get_window_id(&self) -> u32;

    fn draw_new_frame(&self, texture: &RawTextureData);

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

pub struct RetroVideo;

impl Drop for RetroVideo {
    fn drop(&mut self) {
        unsafe {
            let window_ctx = &raw mut WINDOW_CTX;
            window_ctx.replace(None);
        }
    }
}

impl RetroVideo {
    pub fn new() -> Self {
        Self {}
    }

    //noinspection RsPlaceExpression
    pub fn init(&mut self, sdl: &Sdl, av_info: &Arc<AvInfo>) -> Result<(), ErroHandle> {
        match &av_info.video.graphic_api.context_type {
            RETRO_HW_CONTEXT_OPENGL_CORE | RETRO_HW_CONTEXT_OPENGL | RETRO_HW_CONTEXT_NONE => {
                unsafe { WINDOW_CTX = Some(Box::new(GlWindow::new(sdl, av_info)?)) }
                Ok(())
            }
            // RETRO_HW_CONTEXT_VULKAN => {}
            _ => Err(ErroHandle {
                level: retro_log_level::RETRO_LOG_ERROR,
                message: "suporte para a api selecionada não está disponível".to_owned(),
            }),
        }
    }

    pub fn draw_new_frame(&self) {
        unsafe {
            if let Some(window) = &*addr_of_mut!(WINDOW_CTX) {
                window.draw_new_frame(&*addr_of!(RAW_TEX_POINTER))
            }
        }
    }

    pub fn get_window_id(&self) -> u32 {
        unsafe {
            if let Some(window) = &mut *addr_of_mut!(WINDOW_CTX) {
                window.get_window_id()
            } else {
                0
            }
        }
    }

    pub fn resize(&self, new_size: (u32, u32)) {
        unsafe {
            if let Some(window) = &mut *addr_of_mut!(WINDOW_CTX) {
                window.resize(new_size)
            }
        }
    }

    pub fn print_screen(
        &self,
        out_path: &Path,
        av_info: &Arc<AvInfo>,
    ) -> Result<PathBuf, ErroHandle> {
        unsafe {
            PrintScree::take(
                &*addr_of!(RAW_TEX_POINTER),
                av_info,
                &mut PathBuf::from(out_path),
            )
        }
    }

    pub fn disable_full_screen(&self) {
        unsafe {
            if let Some(window) = &mut *addr_of_mut!(WINDOW_CTX) {
                window.disable_full_screen()
            }
        }
    }

    pub fn enable_full_screen(&self) {
        unsafe {
            if let Some(window) = &mut *addr_of_mut!(WINDOW_CTX) {
                window.enable_full_screen()
            }
        }
    }

    pub fn get_core_cb(&self) -> RetroVideoCb {
        RetroVideoCb::default()
    }
}

#[derive(Default)]
pub struct RetroVideoCb;

impl RetroVideoEnvCallbacks for RetroVideoCb {
    fn video_refresh_callback(&self, data: *const c_void, width: u32, height: u32, pitch: usize) {
        unsafe {
            RAW_TEX_POINTER = RawTextureData {
                data,
                height,
                width,
                pitch,
            }
        }
    }

    fn get_proc_address(&self, proc_name: &str) -> *const () {
        unsafe {
            if let Some(window) = &*addr_of!(WINDOW_CTX) {
                window.get_proc_address(proc_name)
            } else {
                null()
            }
        }
    }

    fn context_destroy(&self) {
        unsafe {
            if let Some(window) = &mut *addr_of_mut!(WINDOW_CTX) {
                window.context_destroy();
            }
        }
    }

    fn context_reset(&self) {
        unsafe {
            if let Some(window) = &mut *addr_of_mut!(WINDOW_CTX) {
                window.context_reset();
            }
        }
    }
}
