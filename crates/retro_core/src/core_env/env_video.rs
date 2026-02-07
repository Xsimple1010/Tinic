#[cfg(feature = "hw")]
use crate::libretro_sys::binding_libretro::{
    retro_hw_context_type, retro_hw_render_callback, retro_proc_address_t,
    RETRO_ENVIRONMENT_EXPERIMENTAL, RETRO_ENVIRONMENT_GET_PREFERRED_HW_RENDER,
    RETRO_ENVIRONMENT_SET_HW_RENDER,
};
use libretro_sys::{
    binding_libretro::{
        retro_game_geometry, retro_pixel_format, RETRO_ENVIRONMENT_GET_AUDIO_VIDEO_ENABLE,
        RETRO_ENVIRONMENT_SET_GEOMETRY, RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS,
        RETRO_ENVIRONMENT_SET_PIXEL_FORMAT,
    },
    binding_log_interface,
};
#[cfg(feature = "hw")]
use std::mem;
use std::{
    ffi::{c_char, c_uint, c_void},
    ptr::addr_of,
};

use super::environment::CORE_CONTEXT;

pub unsafe extern "C" fn audio_sample_callback(left: i16, right: i16) {
    if let Some(core_ctx) = &*addr_of!(CORE_CONTEXT) {
        (core_ctx.callbacks.audio_sample_callback)(left, right)
    }
}

pub unsafe extern "C" fn audio_sample_batch_callback(data: *const i16, frames: usize) -> usize {
    if let Some(core_ctx) = &*addr_of!(CORE_CONTEXT) {
        (core_ctx.callbacks.audio_sample_batch_callback)(data, frames)
    } else {
        0
    }
}

pub unsafe extern "C" fn video_refresh_callback(
    data: *const c_void,
    width: c_uint,
    height: c_uint,
    pitch: usize,
) {
    if let Some(core_ctx) = &*addr_of!(CORE_CONTEXT) {
        (core_ctx.callbacks.video_refresh_callback)(data, width, height, pitch);
    }
}

#[cfg(feature = "hw")]
unsafe extern "C" fn get_current_frame_buffer() -> usize {
    println!("get_current_frame_buffer");
    match &*addr_of!(CORE_CONTEXT) {
        Some(core_ctx) => core_ctx
            .av_info
            .video
            .graphic_api
            .fbo
            .read()
            .unwrap()
            .unwrap(),
        None => 0,
    }
}

//TODO: ainda preciso testar  se isso esta funcionando
#[cfg(feature = "hw")]
unsafe extern "C" fn get_proc_address(sym: *const c_char) -> retro_proc_address_t {
    use crate::tools::ffi_tools::get_str_from_ptr;

    println!("get_proc_address");
    match &*addr_of!(CORE_CONTEXT) {
        Some(core_ctx) => {
            let fc_name = get_str_from_ptr(sym);

            let proc_address = (core_ctx.callbacks.get_proc_address)(&fc_name);

            if proc_address.is_null() {
                return None;
            }

            let function: unsafe extern "C" fn() = unsafe { mem::transmute(proc_address) };

            Some(function)
        }
        None => None,
    }
}

#[cfg(feature = "hw")]
unsafe extern "C" fn context_reset() {
    println!("context_reset");
    if let Some(core_ctx) = &*addr_of!(CORE_CONTEXT) {
        (core_ctx.callbacks.context_reset)()
    }
}

#[cfg(feature = "hw")]
unsafe extern "C" fn context_destroy() {
    println!("context_destroy");

    if let Some(core_ctx) = &*addr_of!(CORE_CONTEXT) {
        (core_ctx.callbacks.context_destroy)()
    }
}

pub unsafe fn env_cb_av(cmd: c_uint, data: *mut c_void) -> bool {
    return match cmd {
        RETRO_ENVIRONMENT_SET_GEOMETRY => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_SET_GEOMETRY -> ok");

            let raw_geometry_ptr = data as *mut retro_game_geometry;

            if raw_geometry_ptr.is_null() {
                return false;
            }

            return match &*addr_of!(CORE_CONTEXT) {
                Some(core_ctx) => {
                    core_ctx.av_info.try_set_new_geometry(raw_geometry_ptr);

                    true
                }
                _ => false,
            };
        }
        RETRO_ENVIRONMENT_SET_PIXEL_FORMAT => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_SET_PIXEL_FORMAT -> ok");

            return match &*addr_of!(CORE_CONTEXT) {
                Some(core_ctx) => {
                    *core_ctx.av_info.video.pixel_format.write().unwrap() =
                        *(data as *mut retro_pixel_format);

                    true
                }
                None => false,
            };
        }
        RETRO_ENVIRONMENT_GET_AUDIO_VIDEO_ENABLE => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_GET_AUDIO_VIDEO_ENABLE -> ok");

            *(data as *mut u32) = 1 << 0 | 1 << 1;

            true
        }
        #[cfg(feature = "hw")]
        RETRO_ENVIRONMENT_GET_PREFERRED_HW_RENDER => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_GET_PREFERRED_HW_RENDER");

            return match &*addr_of!(CORE_CONTEXT) {
                Some(core_ctx) => {
                    *(data as *mut retro_hw_context_type) =
                        core_ctx.av_info.video.graphic_api.context_type;

                    true
                }
                _ => false,
            };
        }
        #[cfg(feature = "hw")]
        RETRO_ENVIRONMENT_SET_HW_RENDER | RETRO_ENVIRONMENT_EXPERIMENTAL => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_SET_HW_RENDER");

            if data.is_null() {
                return false;
            }

            binding_log_interface::set_hw_callback(
                data,
                Some(context_reset),
                Some(get_current_frame_buffer),
                Some(context_destroy),
                Some(get_proc_address),
            );

            return match &*addr_of!(CORE_CONTEXT) {
                Some(core_ctx) => core_ctx
                    .av_info
                    .video
                    .graphic_api
                    .try_update_from_raw(data as *mut retro_hw_render_callback),
                _ => false,
            };
        }

        _ => false,
    };
}
