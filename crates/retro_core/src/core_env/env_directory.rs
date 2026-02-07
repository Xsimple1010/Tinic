use crate::{retro_core::RetroCore, tools::ffi_tools::make_c_string};
use generics::constants::MAX_CORE_SUBSYSTEM_INFO;
use libretro_sys::{
    binding_libretro::{
        retro_subsystem_info, RETRO_ENVIRONMENT_GET_CORE_ASSETS_DIRECTORY,
        RETRO_ENVIRONMENT_GET_SAVE_DIRECTORY, RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY,
        RETRO_ENVIRONMENT_GET_VFS_INTERFACE, RETRO_ENVIRONMENT_SET_SUBSYSTEM_INFO,
    },
    binding_log_interface,
};
use std::{ffi::c_uint, os::raw::c_void, sync::Arc};

pub unsafe fn env_cb_directory(core_ctx: &Arc<RetroCore>, cmd: c_uint, data: *mut c_void) -> bool {
    match cmd {
        RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY -> ok");

            let sys_dir = make_c_string(&core_ctx.paths.system).unwrap();

            binding_log_interface::set_directory(data, sys_dir.as_ptr());

            true
        }
        RETRO_ENVIRONMENT_GET_SAVE_DIRECTORY => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_GET_SAVE_DIRECTORY -> ok");

            let save_dir = make_c_string(&core_ctx.paths.save).unwrap();

            binding_log_interface::set_directory(data, save_dir.as_ptr());

            true
        }
        RETRO_ENVIRONMENT_GET_CORE_ASSETS_DIRECTORY => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_GET_CORE_ASSETS_DIRECTORY -> ok");

            let assents_dir = make_c_string(&core_ctx.paths.assets).unwrap();

            binding_log_interface::set_directory(data, assents_dir.as_ptr());

            true
        }
        RETRO_ENVIRONMENT_SET_SUBSYSTEM_INFO => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_SET_SUBSYSTEM_INFO -> OK");

            let raw_subsystem = *(data as *mut [retro_subsystem_info; MAX_CORE_SUBSYSTEM_INFO]);
            core_ctx.system.get_subsystem(raw_subsystem);

            true
        }
        RETRO_ENVIRONMENT_GET_VFS_INTERFACE => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_GET_VFS_INTERFACE -> OK");

            true
        }
        _ => false,
    }
}
