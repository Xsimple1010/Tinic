use crate::{
    core_env::environment::CORE_CONTEXT,
    generics::constants::MAX_CORE_CONTROLLER_INFO_TYPES,
    libretro_sys::binding_libretro::{
        retro_controller_info, retro_rumble_effect, retro_rumble_interface,
        RETRO_ENVIRONMENT_GET_INPUT_BITMASKS, RETRO_ENVIRONMENT_GET_RUMBLE_INTERFACE,
        RETRO_ENVIRONMENT_SET_CONTROLLER_INFO, RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS,
    },
    RetroCoreIns,
};
use std::{ffi::c_uint, os::raw::c_void, ptr::addr_of};

unsafe extern "C" fn rumble_callback(
    port: c_uint,
    effect: retro_rumble_effect,
    strength: u16,
) -> bool {
    match &*addr_of!(CORE_CONTEXT) {
        Some(core_ctx) => core_ctx
            .callbacks
            .controller
            .rumble_callback(port, effect, strength),
        None => false,
    }
}

pub unsafe extern "C" fn input_poll_callback() {
    if let Some(core_ctx) = &*addr_of!(CORE_CONTEXT) {
        core_ctx.callbacks.controller.input_poll_callback()
    }
}

pub unsafe extern "C" fn input_state_callback(
    port: c_uint,
    device: c_uint,
    index: c_uint,
    id: c_uint,
) -> i16 {
    match &*addr_of!(CORE_CONTEXT) {
        Some(core_ctx) => core_ctx.callbacks.controller.input_state_callback(
            port as i16,
            device as i16,
            index as i16,
            id as i16,
        ),
        None => 0,
    }
}

pub unsafe fn env_cb_gamepad_io(core_ctx: &RetroCoreIns, cmd: c_uint, data: *mut c_void) -> bool {
    match cmd {
        RETRO_ENVIRONMENT_GET_INPUT_BITMASKS => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_GET_INPUT_BITMASKS -> ok");
            true
        }
        RETRO_ENVIRONMENT_SET_CONTROLLER_INFO => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_SET_CONTROLLER_INFO -> ok");

            let raw_ctr_infos =
                *(data as *mut [retro_controller_info; MAX_CORE_CONTROLLER_INFO_TYPES]);

            core_ctx.system.get_ports(raw_ctr_infos);

            true
        }
        RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS -> ok");
            false
        }
        RETRO_ENVIRONMENT_GET_RUMBLE_INTERFACE => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_GET_RUMBLE_INTERFACE -> ok");

            let mut rumble_raw = *(data as *mut retro_rumble_interface);
            rumble_raw.set_rumble_state = Some(rumble_callback);

            true
        }

        _ => false,
    }
}
