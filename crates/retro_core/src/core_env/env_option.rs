use libretro_sys::{
    binding_libretro::{
        retro_core_option_display, retro_core_options_v2_intl, retro_variable,
        RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION, RETRO_ENVIRONMENT_GET_VARIABLE,
        RETRO_ENVIRONMENT_GET_VARIABLE_UPDATE, RETRO_ENVIRONMENT_SET_CORE_OPTIONS_DISPLAY,
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_UPDATE_DISPLAY_CALLBACK,
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2_INTL, RETRO_ENVIRONMENT_SET_VARIABLE,
        RETRO_ENVIRONMENT_SET_VARIABLES,
    },
    binding_log_interface,
};
use std::{
    ffi::c_uint,
    os::raw::c_void,
    sync::{atomic::Ordering, Arc},
};

use crate::{
    core::CoreWrapper,
    tools::ffi_tools::{get_str_from_ptr, make_c_string},
};

pub unsafe fn env_cb_option(core_ctx: &Arc<CoreWrapper>, cmd: c_uint, data: *mut c_void) -> bool {
    match cmd {
        RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION -> ok");

            *(data as *mut u32) = 2;

            true
        }
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2_INTL => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2_INTL -> ok");

            let option_intl_v2 = *(data as *mut retro_core_options_v2_intl);

            core_ctx.options.convert_option_v2_intl(option_intl_v2);
            core_ctx.options.try_reload_pref_option();

            true
        }
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_DISPLAY => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_SET_CORE_OPTIONS_DISPLAY -> ok");

            let option = *(data as *mut retro_core_option_display);

            core_ctx
                .options
                .change_visibility(get_str_from_ptr(option.key).as_str(), option.visible);

            true
        }
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_UPDATE_DISPLAY_CALLBACK => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_SET_CORE_OPTIONS_UPDATE_DISPLAY_CALLBACK -> need");
            false
        }
        RETRO_ENVIRONMENT_GET_VARIABLE_UPDATE => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_GET_VARIABLE_UPDATE -> ok");

            *(data as *mut bool) = core_ctx.options.updated_count.load(Ordering::SeqCst) > 0;
            true
        }
        RETRO_ENVIRONMENT_SET_VARIABLES => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_SET_VARIABLES -> needed");
            false
        }
        RETRO_ENVIRONMENT_GET_VARIABLE => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_GET_VARIABLE -> ok");

            let raw_variable = data as *const retro_variable;

            if raw_variable.is_null() {
                return true;
            }

            let options_manager = &core_ctx.options;

            if options_manager.updated_count.load(Ordering::SeqCst) < 1 {
                return false;
            }

            let raw_variable = *(data as *const retro_variable);
            let key = get_str_from_ptr(raw_variable.key);

            match core_ctx.options.get_opt_value(&key) {
                Some(value) => {
                    let new_value = make_c_string(&value).unwrap();

                    binding_log_interface::set_new_value_variable(data, new_value.as_ptr());

                    true
                }
                _ => false,
            }
        }
        RETRO_ENVIRONMENT_SET_VARIABLE => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_SET_VARIABLE -> needed");
            false
        }

        _ => false,
    }
}
