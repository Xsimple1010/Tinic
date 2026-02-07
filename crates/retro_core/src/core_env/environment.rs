#[cfg(feature = "core_logs")]
use crate::tools::ffi_tools::get_str_from_ptr;
use crate::{
    core_env::{
        env_directory::env_cb_directory, env_gamepads_io::env_cb_gamepad_io,
        env_option::env_cb_option, env_video::env_cb_av,
    },
    libretro_sys::{
        binding_libretro::{
            retro_language::{self, RETRO_LANGUAGE_PORTUGUESE_BRAZIL},
            retro_log_level, retro_perf_callback, retro_rumble_effect,
            RETRO_ENVIRONMENT_GET_LANGUAGE, RETRO_ENVIRONMENT_GET_LOG_INTERFACE,
            RETRO_ENVIRONMENT_GET_MESSAGE_INTERFACE_VERSION, RETRO_ENVIRONMENT_GET_PERF_INTERFACE,
            RETRO_ENVIRONMENT_GET_VARIABLE, RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS,
            RETRO_ENVIRONMENT_SET_PERFORMANCE_LEVEL, RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME,
        },
        binding_log_interface::configure_log_interface,
    },
    retro_core::RetroCore,
    retro_perf::{
        core_get_perf_counter, core_perf_log, core_perf_register, core_perf_start, core_perf_stop,
        get_cpu_features, get_features_get_time_usec,
    },
};
use std::{
    ffi::{c_char, c_uint},
    sync::atomic::Ordering,
};
use std::{os::raw::c_void, ptr::addr_of, sync::Arc};

#[derive(Clone, Copy, Debug)]

pub struct RetroEnvCallbacks {
    pub video_refresh_callback: fn(data: *const c_void, width: u32, height: u32, pitch: usize),
    pub audio_sample_callback: fn(left: i16, right: i16),
    pub audio_sample_batch_callback: fn(data: *const i16, frames: usize) -> usize,
    pub input_poll_callback: fn(),
    pub input_state_callback: fn(port: i16, device: i16, index: i16, id: i16) -> i16,
    pub rumble_callback: fn(port: c_uint, effect: retro_rumble_effect, strength: u16) -> bool,
    #[doc = " Called when a context has been created or when it has been reset.\n An OpenGL context is only valid after context_reset() has been called.\n\n When context_reset is called, OpenGL resources in the libretro\n implementation are guaranteed to be invalid.\n\n It is possible that context_reset is called multiple times during an\n application lifecycle.\n If context_reset is called without any notification (context_destroy),\n the OpenGL context was lost and resources should just be recreated\n without any attempt to \"free\" old resources."]
    pub context_reset: fn(),
    #[doc = " Set by frontend.\n Can return all relevant functions, including glClear on Windows."]
    pub get_proc_address: fn(proc_name: &str) -> *const (),
    #[doc = " A callback to be called before the context is destroyed in a\n controlled way by the frontend."]
    pub context_destroy: fn(),
}

#[doc = "pelo amor de deus MANTENHA isso dentro desse diretório"]
pub static mut CORE_CONTEXT: Option<Arc<RetroCore>> = None;

//noinspection RsPlaceExpression
pub fn configure(core_ctx: Arc<RetroCore>) {
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

unsafe extern "C" fn core_log(_level: retro_log_level, _log: *const c_char) {
    #[cfg(feature = "core_logs")]
    println!("[{:?}]: {:?}", _level, get_str_from_ptr(_log));
}

pub unsafe extern "C" fn core_environment(cmd: c_uint, data: *mut c_void) -> bool {
    match &*addr_of!(CORE_CONTEXT) {
        Some(core_ctx) => match cmd {
            RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME => {
                #[cfg(feature = "core_ev_logs")]
                println!("RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME -> ok");

                core_ctx
                    .support_no_game
                    .store(*(data as *mut bool), Ordering::SeqCst);

                true
            }
            RETRO_ENVIRONMENT_GET_LANGUAGE => {
                #[cfg(feature = "core_ev_logs")]
                println!("RETRO_ENVIRONMENT_GET_LANGUAGE -> ok");

                *(data as *mut retro_language) = RETRO_LANGUAGE_PORTUGUESE_BRAZIL;

                true
            }
            RETRO_ENVIRONMENT_GET_LOG_INTERFACE => {
                #[cfg(feature = "core_ev_logs")]
                println!("RETRO_ENVIRONMENT_GET_LOG_INTERFACE -> ok");

                configure_log_interface(Some(core_log), data);

                true
            }
            RETRO_ENVIRONMENT_GET_MESSAGE_INTERFACE_VERSION => {
                #[cfg(feature = "core_ev_logs")]
                println!("RETRO_ENVIRONMENT_SET_SUBSYSTEM_INFO -> OK");

                *(data as *mut usize) = 1;

                true
            }
            RETRO_ENVIRONMENT_SET_PERFORMANCE_LEVEL => {
                #[cfg(feature = "core_ev_logs")]
                println!("RETRO_ENVIRONMENT_SET_PERFORMANCE_LEVEL -> OK");

                core_ctx
                    .system
                    .performance_level
                    .store(*(data as *mut u8), Ordering::SeqCst);

                true
            }
            RETRO_ENVIRONMENT_GET_PERF_INTERFACE => {
                #[cfg(feature = "core_ev_logs")]
                println!("RETRO_ENVIRONMENT_GET_PERF_INTERFACE -> ok");

                let mut perf = *(data as *mut retro_perf_callback);

                perf.get_time_usec = Some(get_features_get_time_usec);
                perf.get_cpu_features = Some(get_cpu_features);
                perf.get_perf_counter = Some(core_get_perf_counter);
                perf.perf_register = Some(core_perf_register);
                perf.perf_start = Some(core_perf_start);
                perf.perf_stop = Some(core_perf_stop);
                perf.perf_log = Some(core_perf_log);

                true
            }
            _ => {
                if env_cb_av(core_ctx, cmd, data)
                    || env_cb_gamepad_io(core_ctx, cmd, data)
                    || env_cb_option(core_ctx, cmd, data)
                    || env_cb_directory(core_ctx, cmd, data)
                {
                    return true;
                }

                if cmd != RETRO_ENVIRONMENT_GET_VARIABLE
                    && cmd != RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS
                {
                    println!("new core cmd -> {:?}", cmd);
                }

                false
            }
        },
        None => false,
    }
}

//TODO: novos teste para "fn core_environment"
#[cfg(test)]
mod test_environment {
    use crate::{core_env::environment::CORE_CONTEXT, test_tools};
    use libretro_sys::binding_libretro::{
        retro_pixel_format, RETRO_ENVIRONMENT_GET_INPUT_BITMASKS,
        RETRO_ENVIRONMENT_SET_PIXEL_FORMAT,
    };
    use std::{ffi::c_void, ptr::addr_of};

    use super::{configure, core_environment};

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
