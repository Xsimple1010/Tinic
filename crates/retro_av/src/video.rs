use crate::{print_scree::PrintScree, retro_gl::window::GlWindow};
use generics::erro_handle::ErroHandle;
use libretro_sys::binding_libretro::{
    retro_hw_context_type::{RETRO_HW_CONTEXT_NONE, RETRO_HW_CONTEXT_OPENGL_CORE},
    retro_log_level,
};
use retro_ab::core::AvInfo;
use sdl2::Sdl;
use std::{
    ffi::{c_uint, c_void},
    path::PathBuf,
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

//noinspection RsPlaceExpression
pub fn video_refresh_callback(data: *const c_void, width: c_uint, height: c_uint, pitch: usize) {
    unsafe {
        RAW_TEX_POINTER = RawTextureData {
            data,
            height,
            width,
            pitch,
        }
    }
}

pub fn get_proc_address(proc_name: &str) -> *const () {
    unsafe {
        if let Some(window) = &*addr_of!(WINDOW_CTX) {
            window.get_proc_address(proc_name)
        } else {
            null()
        }
    }
}

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
}

pub struct RetroVideo {
    av_info: Arc<AvInfo>,
}

impl Drop for RetroVideo {
    fn drop(&mut self) {
        unsafe {
            let window_ctx = &raw mut WINDOW_CTX;
            window_ctx.replace(None);
        }
    }
}

impl RetroVideo {
    //noinspection RsPlaceExpression
    pub fn new(sdl: &Sdl, av_info: &Arc<AvInfo>) -> Result<Self, ErroHandle> {
        match &av_info.video.graphic_api.context_type {
            RETRO_HW_CONTEXT_OPENGL_CORE | RETRO_HW_CONTEXT_NONE => {
                unsafe { WINDOW_CTX = Some(Box::new(GlWindow::new(sdl, av_info)?)) }

                Ok(Self {
                    av_info: av_info.clone(),
                })
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

    pub fn print_screen(&self, out_path: &str, file_name: &str) -> Result<(), ErroHandle> {
        unsafe {
            PrintScree::take(
                &*addr_of!(RAW_TEX_POINTER),
                &self.av_info,
                &mut PathBuf::from(out_path),
                file_name,
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

    pub fn full_screen(&self) {
        unsafe {
            if let Some(window) = &mut *addr_of_mut!(WINDOW_CTX) {
                window.enable_full_screen()
            }
        }
    }
}
