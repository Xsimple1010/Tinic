use crate::{
    core_env::environment::CORE_CONTEXT,
    generics::constants::MAX_CORE_CONTROLLER_INFO_TYPES,
    libretro_sys::binding_libretro::{
        retro_controller_info, retro_rumble_effect,
        retro_rumble_interface, RETRO_ENVIRONMENT_GET_INPUT_BITMASKS,
        RETRO_ENVIRONMENT_GET_RUMBLE_INTERFACE, RETRO_ENVIRONMENT_SET_CONTROLLER_INFO, RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS,
    },
    tools::validation::InputValidator,
    RetroCoreIns,
};
use generics::error_handle::ErrorHandle;
use std::{ffi::c_uint, os::raw::c_void, ptr::addr_of};

unsafe extern "C" fn rumble_callback(
    port: c_uint,
    effect: retro_rumble_effect,
    strength: u16,
) -> bool {
    let retro_core = unsafe {
        match &*addr_of!(CORE_CONTEXT) {
            Some(core_ctx) => core_ctx,
            None => return false,
        }
    };

    let res = retro_core
        .callbacks
        .controller
        .rumble_callback(port, effect, strength);

    match res {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e);
            let _ = retro_core.de_init();
            false
        }
    }
}

pub unsafe extern "C" fn input_poll_callback() {
    let retro_core = unsafe {
        match &*addr_of!(CORE_CONTEXT) {
            Some(core_ctx) => core_ctx,
            None => return,
        }
    };

    if let Err(e) = retro_core.callbacks.controller.input_poll_callback() {
        println!("{:?}", e);
        let _ = retro_core.de_init();
    }
}

pub unsafe extern "C" fn input_state_callback(
    port: c_uint,
    device: c_uint,
    index: c_uint,
    id: c_uint,
) -> i16 {
    let retro_core = unsafe {
        match &*addr_of!(CORE_CONTEXT) {
            Some(core_ctx) => core_ctx,
            None => return 0,
        }
    };

    let res = retro_core.callbacks.controller.input_state_callback(
        port as i16,
        device as i16,
        index as i16,
        id as i16,
    );

    match res {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e);
            let _ = retro_core.de_init();
            0
        }
    }
}

pub unsafe fn env_cb_gamepad_io(
    core_ctx: &RetroCoreIns,
    cmd: c_uint,
    data: *mut c_void,
) -> Result<bool, ErrorHandle> {
    match cmd {
        RETRO_ENVIRONMENT_GET_INPUT_BITMASKS => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_GET_INPUT_BITMASKS -> ok");
            Ok(true)
        }
        RETRO_ENVIRONMENT_SET_CONTROLLER_INFO => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_SET_CONTROLLER_INFO -> ok");

            InputValidator::validate_non_null_mut_ptr(
                data,
                "data in RETRO_ENVIRONMENT_SET_CONTROLLER_INFO",
            )?;

            let raw_ctr_infos =
                unsafe { *(data as *mut [retro_controller_info; MAX_CORE_CONTROLLER_INFO_TYPES]) };

            let _ = core_ctx.system.get_ports(raw_ctr_infos);

            Ok(true)
        }
        RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS -> ok");
            Ok(false)
        }
        RETRO_ENVIRONMENT_GET_RUMBLE_INTERFACE => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_GET_RUMBLE_INTERFACE -> ok");

            InputValidator::validate_non_null_mut_ptr(
                data,
                "data in RETRO_ENVIRONMENT_GET_RUMBLE_INTERFACE",
            )?;

            let rumble_raw = unsafe { &mut *(data as *mut retro_rumble_interface) };
            rumble_raw.set_rumble_state = Some(rumble_callback);

            Ok(true)
        }

        _ => Ok(false),
    }
}
