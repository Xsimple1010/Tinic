use crate::{RetroCoreIns, tools::ffi_tools::make_c_string};
use generics::constants::MAX_CORE_SUBSYSTEM_INFO;
use generics::error_handle::ErrorHandle;
use libretro_sys::{
    binding_libretro::{
        RETRO_ENVIRONMENT_GET_CORE_ASSETS_DIRECTORY, RETRO_ENVIRONMENT_GET_SAVE_DIRECTORY,
        RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY, RETRO_ENVIRONMENT_GET_VFS_INTERFACE,
        RETRO_ENVIRONMENT_SET_SUBSYSTEM_INFO, retro_subsystem_info,
    },
    binding_log_interface,
};
use std::{ffi::c_uint, os::raw::c_void};

pub unsafe fn env_cb_directory(
    core_ctx: &RetroCoreIns,
    cmd: c_uint,
    data: *mut c_void,
) -> Result<bool, ErrorHandle> {
    match cmd {
        RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY -> ok");

            let sys_dir = make_c_string(
                &core_ctx.paths.system,
                "Nao foi possivel cria uma C String de sys_dir para enviar ao core",
            )?;

            unsafe {
                binding_log_interface::set_directory(data, sys_dir.as_ptr());
            }

            Ok(true)
        }
        RETRO_ENVIRONMENT_GET_SAVE_DIRECTORY => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_GET_SAVE_DIRECTORY -> ok");

            let save_dir = make_c_string(
                &core_ctx.paths.save,
                "Nao foi possivel cria uma C String de save_dir para enviar ao core",
            )?;

            unsafe {
                binding_log_interface::set_directory(data, save_dir.as_ptr());
            }

            Ok(true)
        }
        RETRO_ENVIRONMENT_GET_CORE_ASSETS_DIRECTORY => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_GET_CORE_ASSETS_DIRECTORY -> ok");

            let assents_dir = make_c_string(
                &core_ctx.paths.assets,
                "Nao foi possivel cria uma C String de assents_dir para enviar ao core",
            )?;

            unsafe {
                binding_log_interface::set_directory(data, assents_dir.as_ptr());
            }

            Ok(true)
        }
        RETRO_ENVIRONMENT_SET_SUBSYSTEM_INFO => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_SET_SUBSYSTEM_INFO -> OK");

            let raw_subsystem =
                unsafe { *(data as *mut [retro_subsystem_info; MAX_CORE_SUBSYSTEM_INFO]) };
            core_ctx.system.get_subsystem(raw_subsystem)?;

            Ok(true)
        }
        RETRO_ENVIRONMENT_GET_VFS_INTERFACE => {
            #[cfg(feature = "core_ev_logs")]
            println!("RETRO_ENVIRONMENT_GET_VFS_INTERFACE -> OK");

            Ok(true)
        }
        _ => Ok(false),
    }
}
