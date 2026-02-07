use crate::{
    core::CoreWrapper,
    libretro_sys::binding_libretro::{
        retro_controller_info, retro_core_option_display, retro_core_options_v2_intl,
        retro_game_geometry, retro_hw_context_type, retro_hw_render_callback, retro_language,
        retro_log_level, retro_perf_callback, retro_pixel_format, retro_proc_address_t,
        retro_rumble_effect, retro_rumble_interface, retro_subsystem_info, retro_variable,
        RETRO_ENVIRONMENT_GET_AUDIO_VIDEO_ENABLE, RETRO_ENVIRONMENT_GET_CORE_ASSETS_DIRECTORY,
        RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION,
        RETRO_ENVIRONMENT_GET_DISK_CONTROL_INTERFACE_VERSION, RETRO_ENVIRONMENT_GET_INPUT_BITMASKS,
        RETRO_ENVIRONMENT_GET_LANGUAGE, RETRO_ENVIRONMENT_GET_LED_INTERFACE,
        RETRO_ENVIRONMENT_GET_LOG_INTERFACE, RETRO_ENVIRONMENT_GET_MESSAGE_INTERFACE_VERSION,
        RETRO_ENVIRONMENT_GET_PERF_INTERFACE, RETRO_ENVIRONMENT_GET_PREFERRED_HW_RENDER,
        RETRO_ENVIRONMENT_GET_RUMBLE_INTERFACE, RETRO_ENVIRONMENT_GET_SAVE_DIRECTORY,
        RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY, RETRO_ENVIRONMENT_GET_USERNAME,
        RETRO_ENVIRONMENT_GET_VARIABLE, RETRO_ENVIRONMENT_GET_VARIABLE_UPDATE,
        RETRO_ENVIRONMENT_GET_VFS_INTERFACE, RETRO_ENVIRONMENT_SET_CONTROLLER_INFO,
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_DISPLAY,
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_UPDATE_DISPLAY_CALLBACK,
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2_INTL, RETRO_ENVIRONMENT_SET_DISK_CONTROL_INTERFACE,
        RETRO_ENVIRONMENT_SET_GEOMETRY, RETRO_ENVIRONMENT_SET_HW_RENDER,
        RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS, RETRO_ENVIRONMENT_SET_PERFORMANCE_LEVEL,
        RETRO_ENVIRONMENT_SET_PIXEL_FORMAT, RETRO_ENVIRONMENT_SET_SERIALIZATION_QUIRKS,
        RETRO_ENVIRONMENT_SET_SUBSYSTEM_INFO, RETRO_ENVIRONMENT_SET_SUPPORT_ACHIEVEMENTS,
        RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME, RETRO_ENVIRONMENT_SET_VARIABLE,
        RETRO_ENVIRONMENT_SET_VARIABLES,
    },
    retro_context::RetroContext,
    retro_perf::{
        core_get_perf_counter, core_perf_log, core_perf_register, core_perf_start, core_perf_stop,
        get_cpu_features, get_features_get_time_usec,
    },
    tools::ffi_tools::{get_str_from_ptr, make_c_string},
};
use ::std::os::raw;
use generics::constants::{MAX_CORE_CONTROLLER_INFO_TYPES, MAX_CORE_SUBSYSTEM_INFO};
use libretro_sys::binding_log_interface;
use std::mem;
use std::sync::atomic::Ordering;
use std::{os::raw::c_void, ptr::addr_of, sync::Arc};

#[derive(Clone, Copy, Debug)]
pub struct RetroEnvCallbacks {
    pub video_refresh_callback: fn(data: *const c_void, width: u32, height: u32, pitch: usize),
    pub audio_sample_callback: fn(left: i16, right: i16),
    pub audio_sample_batch_callback: fn(data: *const i16, frames: usize) -> usize,
    pub input_poll_callback: fn(),
    pub input_state_callback: fn(port: i16, device: i16, index: i16, id: i16) -> i16,
    pub rumble_callback: fn(port: raw::c_uint, effect: retro_rumble_effect, strength: u16) -> bool,
    #[doc = " Called when a context has been created or when it has been reset.\n An OpenGL context is only valid after context_reset() has been called.\n\n When context_reset is called, OpenGL resources in the libretro\n implementation are guaranteed to be invalid.\n\n It is possible that context_reset is called multiple times during an\n application lifecycle.\n If context_reset is called without any notification (context_destroy),\n the OpenGL context was lost and resources should just be recreated\n without any attempt to \"free\" old resources."]
    pub context_reset: fn(),
    #[doc = " Set by frontend.\n Can return all relevant functions, including glClear on Windows."]
    pub get_proc_address: fn(proc_name: &str) -> *const (),
    #[doc = " A callback to be called before the context is destroyed in a\n controlled way by the frontend."]
    pub context_destroy: fn(),
}

static mut CORE_CONTEXT: Option<Arc<CoreWrapper>> = None;

//noinspection RsPlaceExpression
pub fn configure(core_ctx: Arc<CoreWrapper>) {
    unsafe {
        CORE_CONTEXT = Some(core_ctx);
    }
}

//noinspection RsPlaceExpression
pub fn delete_local_core_ctx() {
    unsafe {
        CORE_CONTEXT = None;
    }
}

fn _force_stop() {
    unsafe {
        if let Some(core_ctx) = &*addr_of!(CORE_CONTEXT) {
            let retro_ctx = RetroContext::get_from_id(&core_ctx.retro_ctx_associated)
                .expect("não foi possível forca o fechamento");

            retro_ctx.delete().unwrap();
        }
    }
}

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

pub unsafe extern "C" fn input_poll_callback() {
    if let Some(core_ctx) = &*addr_of!(CORE_CONTEXT) {
        (core_ctx.callbacks.input_poll_callback)()
    }
}

pub unsafe extern "C" fn input_state_callback(
    port: raw::c_uint,
    device: raw::c_uint,
    index: raw::c_uint,
    id: raw::c_uint,
) -> i16 {
    match &*addr_of!(CORE_CONTEXT) {
        Some(core_ctx) => (core_ctx.callbacks.input_state_callback)(
            port as i16,
            device as i16,
            index as i16,
            id as i16,
        ),
        None => 0,
    }
}

pub unsafe extern "C" fn video_refresh_callback(
    data: *const c_void,
    width: raw::c_uint,
    height: raw::c_uint,
    pitch: usize,
) {
    if let Some(core_ctx) = &*addr_of!(CORE_CONTEXT) {
        (core_ctx.callbacks.video_refresh_callback)(data, width, height, pitch);
    }
}

unsafe extern "C" fn rumble_callback(
    port: raw::c_uint,
    effect: retro_rumble_effect,
    strength: u16,
) -> bool {
    match &*addr_of!(CORE_CONTEXT) {
        Some(core_ctx) => (core_ctx.callbacks.rumble_callback)(port, effect, strength),
        None => false,
    }
}

unsafe extern "C" fn core_log(_level: retro_log_level, _log: *const raw::c_char) {
    #[cfg(feature = "core_logs")]
    println!("[{:?}]: {:?}", _level, get_str_from_ptr(_log));
}

unsafe extern "C" fn get_current_frame_buffer() -> usize {
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
unsafe extern "C" fn get_proc_address(sym: *const raw::c_char) -> retro_proc_address_t {
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

unsafe extern "C" fn context_reset() {
    if let Some(core_ctx) = &*addr_of!(CORE_CONTEXT) {
        (core_ctx.callbacks.context_reset)()
    }
}

unsafe extern "C" fn context_destroy() {
    if let Some(core_ctx) = &*addr_of!(CORE_CONTEXT) {
        (core_ctx.callbacks.context_destroy)()
    }
}

pub unsafe extern "C" fn core_environment(cmd: raw::c_uint, data: *mut c_void) -> bool {
    match cmd {
        RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME -> ok");

            match &*addr_of!(CORE_CONTEXT) {
                Some(core_ctx) => {
                    core_ctx
                        .support_no_game
                        .store(*(data as *mut bool), Ordering::SeqCst);
                }
                None => return false,
            }

            return true;
        }
        RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY -> ok");

            match &*addr_of!(CORE_CONTEXT) {
                Some(core_ctx) => {
                    let sys_dir = make_c_string(&core_ctx.paths.system).unwrap();

                    binding_log_interface::set_directory(data, sys_dir.as_ptr())
                }
                _ => return false,
            }

            return true;
        }
        RETRO_ENVIRONMENT_GET_SAVE_DIRECTORY => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_GET_SAVE_DIRECTORY -> ok");

            match &*addr_of!(CORE_CONTEXT) {
                Some(core_ctx) => {
                    let save_dir = make_c_string(&core_ctx.paths.save).unwrap();

                    binding_log_interface::set_directory(data, save_dir.as_ptr())
                }
                _ => return false,
            }

            return true;
        }
        RETRO_ENVIRONMENT_GET_CORE_ASSETS_DIRECTORY => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_GET_CORE_ASSETS_DIRECTORY -> ok");

            match &*addr_of!(CORE_CONTEXT) {
                Some(core_ctx) => {
                    let assents_dir = make_c_string(&core_ctx.paths.assets).unwrap();

                    binding_log_interface::set_directory(data, assents_dir.as_ptr())
                }
                _ => return false,
            }

            return true;
        }
        RETRO_ENVIRONMENT_SET_SUPPORT_ACHIEVEMENTS => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_SET_SUPPORT_ACHIEVEMENTS");
        }
        RETRO_ENVIRONMENT_SET_PERFORMANCE_LEVEL => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_SET_PERFORMANCE_LEVEL");
        }
        RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION -> ok");
            *(data as *mut u32) = 2;
            return true;
        }
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2_INTL => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2_INTL -> ok");

            match &*addr_of!(CORE_CONTEXT) {
                Some(core_ctx) => {
                    let option_intl_v2 = *(data as *mut retro_core_options_v2_intl);

                    core_ctx.options.convert_option_v2_intl(option_intl_v2);
                    core_ctx.options.try_reload_pref_option();
                }
                _ => return false,
            }

            return true;
        }
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_DISPLAY => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_SET_CORE_OPTIONS_DISPLAY -> ok");

            match &*addr_of!(CORE_CONTEXT) {
                Some(core_ctx) => {
                    let option = *(data as *mut retro_core_option_display);

                    core_ctx
                        .options
                        .change_visibility(get_str_from_ptr(option.key).as_str(), option.visible)
                }
                _ => return false,
            }

            return true;
        }
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_UPDATE_DISPLAY_CALLBACK => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_SET_CORE_OPTIONS_UPDATE_DISPLAY_CALLBACK");
        }
        RETRO_ENVIRONMENT_GET_LANGUAGE => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_GET_LANGUAGE -> ok");
            match &*addr_of!(CORE_CONTEXT) {
                Some(core_ctx) => {
                    *(data as *mut retro_language) = *core_ctx.language.lock().unwrap();
                }
                None => return false,
            }
            return true;
        }
        RETRO_ENVIRONMENT_SET_GEOMETRY => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_SET_GEOMETRY -> ok");
            let raw_geometry_ptr = data as *mut retro_game_geometry;

            if raw_geometry_ptr.is_null() {
                return false;
            }

            match &*addr_of!(CORE_CONTEXT) {
                Some(core_ctx) => {
                    core_ctx.av_info.try_set_new_geometry(raw_geometry_ptr);
                }
                _ => return false,
            }

            return true;
        }
        RETRO_ENVIRONMENT_SET_PIXEL_FORMAT => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_SET_PIXEL_FORMAT -> ok");

            match &*addr_of!(CORE_CONTEXT) {
                Some(core_ctx) => {
                    *core_ctx.av_info.video.pixel_format.write().unwrap() =
                        *(data as *mut retro_pixel_format);
                }
                None => return false,
            }
            return true;
        }
        RETRO_ENVIRONMENT_GET_VARIABLE_UPDATE => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_GET_VARIABLE_UPDATE -> ok");

            match &*addr_of!(CORE_CONTEXT) {
                Some(core_ctx) => {
                    if core_ctx.options.updated_count.load(Ordering::SeqCst) > 0 {
                        *(data as *mut bool) = true;
                    } else {
                        *(data as *mut bool) = false;
                    }
                }
                _ => return false,
            }

            return true;
        }
        RETRO_ENVIRONMENT_SET_VARIABLES => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_SET_VARIABLES");
        }
        RETRO_ENVIRONMENT_GET_VARIABLE => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_GET_VARIABLE -> ok");

            let raw_variable = data as *const retro_variable;

            if raw_variable.is_null() {
                return true;
            }

            return match &*addr_of!(CORE_CONTEXT) {
                Some(core_ctx) => {
                    let options_manager = &core_ctx.options;

                    if options_manager.updated_count.load(Ordering::SeqCst) < 1 {
                        return false;
                    }

                    let raw_variable = *(data as *const retro_variable);
                    let key = get_str_from_ptr(raw_variable.key);

                    return match core_ctx.options.get_opt_value(&key) {
                        Some(value) => {
                            let new_value = make_c_string(&value).unwrap();

                            return binding_log_interface::set_new_value_variable(
                                data,
                                new_value.as_ptr(),
                            );
                        }
                        _ => false,
                    };
                }
                _ => false,
            };
        }
        RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS");
        }
        RETRO_ENVIRONMENT_GET_LOG_INTERFACE => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_GET_LOG_INTERFACE -> ok");

            binding_log_interface::configure_log_interface(Some(core_log), data);

            return true;
        }
        RETRO_ENVIRONMENT_SET_SUBSYSTEM_INFO => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_SET_SUBSYSTEM_INFO -> OK");

            match &*addr_of!(CORE_CONTEXT) {
                Some(core_ctx) => {
                    let raw_subsystem =
                        *(data as *mut [retro_subsystem_info; MAX_CORE_SUBSYSTEM_INFO]);

                    core_ctx.system.get_subsystem(raw_subsystem)
                }
                None => return false,
            }

            return true;
        }
        RETRO_ENVIRONMENT_GET_INPUT_BITMASKS => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_GET_INPUT_BITMASKS -> ok");
            return true;
        }
        RETRO_ENVIRONMENT_SET_CONTROLLER_INFO => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_SET_CONTROLLER_INFO -> ok");

            match &*addr_of!(CORE_CONTEXT) {
                Some(core_ctx) => {
                    let raw_ctr_infos =
                        *(data as *mut [retro_controller_info; MAX_CORE_CONTROLLER_INFO_TYPES]);

                    core_ctx.system.get_ports(raw_ctr_infos);
                }
                _ => return false,
            }

            return true;
        }
        RETRO_ENVIRONMENT_GET_AUDIO_VIDEO_ENABLE => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_GET_AUDIO_VIDEO_ENABLE -> ok");

            *(data as *mut u32) = 1 << 0 | 1 << 1;

            return true;
        }
        RETRO_ENVIRONMENT_GET_VFS_INTERFACE => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_GET_VFS_INTERFACE");
        }
        RETRO_ENVIRONMENT_GET_LED_INTERFACE => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_GET_LED_INTERFACE");
        }
        RETRO_ENVIRONMENT_GET_MESSAGE_INTERFACE_VERSION => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_GET_MESSAGE_INTERFACE_VERSION");
        }
        RETRO_ENVIRONMENT_GET_DISK_CONTROL_INTERFACE_VERSION => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_GET_DISK_CONTROL_INTERFACE_VERSION");
        }
        RETRO_ENVIRONMENT_SET_DISK_CONTROL_INTERFACE => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_SET_DISK_CONTROL_INTERFACE");
        }
        RETRO_ENVIRONMENT_GET_PERF_INTERFACE => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_GET_PERF_INTERFACE -> ok");

            let mut perf = *(data as *mut retro_perf_callback);

            perf.get_time_usec = Some(get_features_get_time_usec);
            perf.get_cpu_features = Some(get_cpu_features);
            perf.get_perf_counter = Some(core_get_perf_counter);
            perf.perf_register = Some(core_perf_register);
            perf.perf_start = Some(core_perf_start);
            perf.perf_stop = Some(core_perf_stop);
            perf.perf_log = Some(core_perf_log);

            return true;
        }
        RETRO_ENVIRONMENT_SET_SERIALIZATION_QUIRKS => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_SET_SERIALIZATION_QUIRKS");
        }
        RETRO_ENVIRONMENT_GET_PREFERRED_HW_RENDER => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_GET_PREFERRED_HW_RENDER");

            match &*addr_of!(CORE_CONTEXT) {
                Some(core_ctx) => {
                    *(data as *mut retro_hw_context_type) =
                        core_ctx.av_info.video.graphic_api.context_type
                }
                _ => return false,
            }

            return true;
        }
        RETRO_ENVIRONMENT_SET_HW_RENDER => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_SET_HW_RENDER");

            if data.is_null() {
                return false;
            }

            libretro_sys::binding_log_interface::set_hw_callback(
                data,
                Some(context_reset),
                Some(get_current_frame_buffer),
                Some(context_destroy),
                Some(get_proc_address),
            );

                    core_ctx
                        .av_info
                        .video
                        .graphic_api
                        .stencil
                        .store(data.stencil, Ordering::SeqCst);

                    core_ctx
                        .av_info
                        .video
                        .graphic_api
                        .bottom_left_origin
                        .store(data.bottom_left_origin, Ordering::SeqCst);

                    core_ctx
                        .av_info
                        .video
                        .graphic_api
                        .minor
                        .store(data.version_minor, Ordering::SeqCst);

                    core_ctx
                        .av_info
                        .video
                        .graphic_api
                        .major
                        .store(data.version_major, Ordering::SeqCst);

                    core_ctx
                        .av_info
                        .video
                        .graphic_api
                        .cache_context
                        .store(data.cache_context, Ordering::SeqCst);

                    core_ctx
                        .av_info
                        .video
                        .graphic_api
                        .debug_context
                        .store(data.debug_context, Ordering::SeqCst);

                    data.get_current_framebuffer = Some(get_current_frame_buffer);

                    data.get_proc_address = Some(get_proc_address);
                    data.context_reset = Some(context_reset);
                    data.context_destroy = Some(context_destroy);
                }
                _ => return false,
            }

            return true;
        }
        RETRO_ENVIRONMENT_SET_VARIABLE => {
            // #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_SET_VARIABLE");
        }
        RETRO_ENVIRONMENT_GET_USERNAME => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_GET_USERNAME");
        }
        RETRO_ENVIRONMENT_GET_RUMBLE_INTERFACE => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_GET_RUMBLE_INTERFACE -> ok");

            let mut rumble_raw = *(data as *mut retro_rumble_interface);
            rumble_raw.set_rumble_state = Some(rumble_callback);

            return true;
        }
        _ => {
            println!("{:?}", cmd);
            return false;
        }
    }
    false
}

//TODO: novos teste para "fn core_environment"
#[cfg(test)]
mod test_environment {
    use crate::{
        environment::{configure, CORE_CONTEXT},
        test_tools,
    };
    use libretro_sys::binding_libretro::{
        retro_pixel_format, RETRO_ENVIRONMENT_GET_INPUT_BITMASKS,
        RETRO_ENVIRONMENT_SET_PIXEL_FORMAT,
    };
    use std::{ffi::c_void, ptr::addr_of};

    use super::core_environment;

    fn cfg_test() {
        let core_ctx = test_tools::core::get_core_wrapper();
        configure(core_ctx);
    }

    #[test]
    fn input_bitmasks() {
        let my_bool = true;
        let data = &my_bool as *const bool as *mut c_void;

        let result = unsafe { core_environment(RETRO_ENVIRONMENT_GET_INPUT_BITMASKS, data) };

        assert_eq!(result, true);

        assert_eq!(my_bool, true);
    }

    #[test]
    fn pixel_format() {
        cfg_test();
        let pixel = retro_pixel_format::RETRO_PIXEL_FORMAT_RGB565;
        let data = &pixel as *const retro_pixel_format as *mut c_void;

        let result = unsafe { core_environment(RETRO_ENVIRONMENT_SET_PIXEL_FORMAT, data) };

        assert_eq!(
            result, true,
            "returno inesperado: valor desejado -> true; valor recebido -> {:?}",
            result,
        );

        unsafe {
            match &*addr_of!(CORE_CONTEXT) {
                Some(core_ctx) => assert_eq!(
                    *core_ctx.av_info.video.pixel_format.read().unwrap(),
                    pixel,
                    "returno inesperado: valor desejado -> {:?}; valor recebido -> {:?}",
                    pixel,
                    *core_ctx.av_info.video.pixel_format.read().unwrap()
                ),
                _ => panic!("CORE_CONTEXT nao foi encontrado"),
            }
        }
    }
}
