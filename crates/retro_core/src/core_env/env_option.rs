use crate::{
    RetroCoreIns,
    tools::ffi_tools::{get_str_from_ptr, make_c_string},
};
use generics::error_handle::ErrorHandle;
use libretro_sys::{
    binding_libretro::{
        RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION, RETRO_ENVIRONMENT_GET_VARIABLE,
        RETRO_ENVIRONMENT_GET_VARIABLE_UPDATE, RETRO_ENVIRONMENT_SET_CORE_OPTIONS_DISPLAY,
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_INTL,
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_UPDATE_DISPLAY_CALLBACK,
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2_INTL, RETRO_ENVIRONMENT_SET_VARIABLE,
        RETRO_ENVIRONMENT_SET_VARIABLES, retro_core_option_display, retro_core_options_v2_intl,
        retro_variable,
    },
    binding_log_interface,
};
use std::{ffi::c_uint, os::raw::c_void, sync::atomic::Ordering};

pub unsafe fn env_cb_option(
    core_ctx: &RetroCoreIns,
    cmd: c_uint,
    data: *mut c_void,
) -> Result<bool, ErrorHandle> {
    match cmd {
        RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION -> ok");

            unsafe {
                *(data as *mut u32) = 2;
            }

            Ok(true)
        }
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_INTL => {
            println!("RETRO_ENVIRONMENT_SET_CORE_OPTIONS_INTL");

            Ok(false)
        }
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2_INTL => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2_INTL -> ok");

            let option_intl_v2 = unsafe { *(data as *mut retro_core_options_v2_intl) };

            let _ = core_ctx.options.convert_option_v2_intl(option_intl_v2);
            let _ = core_ctx.options.try_reload_pref_option();

            Ok(true)
        }
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_DISPLAY => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_SET_CORE_OPTIONS_DISPLAY -> ok");

            let option = unsafe { *(data as *mut retro_core_option_display) };

            let _ = core_ctx
                .options
                .change_visibility(&get_str_from_ptr(option.key), option.visible);

            Ok(true)
        }
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_UPDATE_DISPLAY_CALLBACK => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_SET_CORE_OPTIONS_UPDATE_DISPLAY_CALLBACK -> need");
            Ok(false)
        }
        RETRO_ENVIRONMENT_GET_VARIABLE_UPDATE => {
            #[cfg(feature = "core_ev_logs")]
            println!(
                "RETRO_ENVIRONMENT_GET_VARIABLE_UPDATE {:?} -> ok",
                core_ctx.options.updated_count.load(Ordering::SeqCst) > 0
            );

            unsafe {
                *(data as *mut bool) = core_ctx.options.updated_count.load(Ordering::SeqCst) > 0;
            }

            Ok(true)
        }
        RETRO_ENVIRONMENT_SET_VARIABLES => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_SET_VARIABLES -> needed");
            Ok(false)
        }
        RETRO_ENVIRONMENT_SET_VARIABLE => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_SET_VARIABLE -> needed");
            Ok(false)
        }
        RETRO_ENVIRONMENT_GET_VARIABLE => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_GET_VARIABLE -> ok");

            let raw_variable = data as *const retro_variable;

            if raw_variable.is_null() {
                return Ok(true);
            }

            let options_manager = &core_ctx.options;

            if options_manager.updated_count.load(Ordering::SeqCst) < 1 {
                return Ok(false);
            }

            let raw_variable = unsafe { *(data as *const retro_variable) };
            let key = get_str_from_ptr(raw_variable.key);

            match options_manager.get_opt_value(&key)? {
                Some(value) => {
                    let new_value = make_c_string(
                        &value,
                        "Nao foi possivel cria uma C String do novo valor de core_opt",
                    )?;

                    unsafe {
                        binding_log_interface::set_new_value_variable(data, new_value.as_ptr());
                    }

                    Ok(true)
                }
                _ => Ok(false),
            }
        }
        _ => Ok(false),
    }
}
