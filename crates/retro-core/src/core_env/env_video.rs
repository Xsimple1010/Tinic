#[cfg(feature = "hw")]
use crate::libretro_sys::binding_libretro::{
    RETRO_ENVIRONMENT_GET_HW_RENDER_INTERFACE, RETRO_ENVIRONMENT_GET_PREFERRED_HW_RENDER,
    RETRO_ENVIRONMENT_SET_HW_RENDER, RETRO_ENVIRONMENT_SET_HW_SHARED_CONTEXT,
    retro_hw_context_type, retro_hw_render_callback,
};
use crate::{
    RetroCoreIns,
    libretro_sys::binding_libretro::{
        RETRO_ENVIRONMENT_GET_AUDIO_VIDEO_ENABLE, RETRO_ENVIRONMENT_SET_GEOMETRY,
        RETRO_ENVIRONMENT_SET_PIXEL_FORMAT, retro_game_geometry, retro_pixel_format,
    },
    pixel::PixelFormat,
    tools::validation::InputValidator,
};
use std::ffi::{c_uint, c_void};
use tinic_generics::error_handle::ErrorHandle;

pub unsafe fn env_cb_av(
    core_ctx: &RetroCoreIns,
    cmd: c_uint,
    data: *mut c_void,
) -> Result<bool, ErrorHandle> {
    match cmd {
        RETRO_ENVIRONMENT_SET_GEOMETRY => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_SET_GEOMETRY -> ok");

            InputValidator::validate_non_null_ptr(
                data,
                "ptr data in RETRO_ENVIRONMENT_SET_GEOMETRY",
            )?;

            let raw_geometry_ptr = unsafe { &*(data as *const retro_game_geometry) };

            core_ctx.av_info.try_set_new_geometry(raw_geometry_ptr)?;

            Ok(true)
        }
        RETRO_ENVIRONMENT_SET_PIXEL_FORMAT => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_SET_PIXEL_FORMAT -> ok");

            InputValidator::validate_non_null_ptr(
                data,
                "ptr data in RETRO_ENVIRONMENT_SET_PIXEL_FORMAT",
            )?;

            unsafe {
                core_ctx.av_info.video.pixel_format.store_or_else(
                    (*(data as *const retro_pixel_format)).into(),
                    |p| {
                        let mut _pixel = *p.into_inner();
                        _pixel = PixelFormat::Unknown;
                    },
                );
            }

            Ok(true)
        }
        RETRO_ENVIRONMENT_GET_AUDIO_VIDEO_ENABLE => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_GET_AUDIO_VIDEO_ENABLE -> ok");

            unsafe {
                *(data as *mut u32) = 1 << 0 | 1 << 1;
            }

            Ok(true)
        }
        #[cfg(feature = "hw")]
        RETRO_ENVIRONMENT_GET_PREFERRED_HW_RENDER => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_GET_PREFERRED_HW_RENDER");

            unsafe {
                *(data as *mut retro_hw_context_type) = core_ctx
                    .av_info
                    .video
                    .graphic_api
                    .context_type
                    .read()?
                    .clone()
                    .into();
            }

            Ok(true)
        }
        #[cfg(feature = "hw")]
        RETRO_ENVIRONMENT_SET_HW_RENDER => unsafe {
            use crate::core_env::env_callbacks::{get_current_frame_buffer, get_proc_address};

            let hw_cb = &mut *(data as *mut retro_hw_render_callback);

            println!(
                "SET_HW_RENDER: context_type={:?} major={} minor={} depth={} stencil={}",
                hw_cb.context_type,
                hw_cb.version_major,
                hw_cb.version_minor,
                hw_cb.depth,
                hw_cb.stencil
            );

            hw_cb.get_current_framebuffer = Some(get_current_frame_buffer);
            hw_cb.get_proc_address = Some(get_proc_address);

            Ok(core_ctx
                .av_info
                .video
                .graphic_api
                .try_update_from_raw(hw_cb)?)
        },
        #[cfg(feature = "hw")]
        RETRO_ENVIRONMENT_SET_HW_SHARED_CONTEXT => {
            // Retorna true para indicar que suportamos shared context
            // Isso faz o PPSSPP usar glewExperimental = GL_TRUE
            println!("RETRO_ENVIRONMENT_SET_HW_SHARED_CONTEXT");
            Ok(true)
        }
        #[cfg(feature = "hw")]
        RETRO_ENVIRONMENT_GET_HW_RENDER_INTERFACE => {
            println!("RETRO_ENVIRONMENT_GET_HW_RENDER_INTERFACE");
            Ok(false) // não suportado por enquanto
        }
        _ => Ok(false),
    }
}
