use super::ffi_tools::make_c_string;
use crate::core::CoreWrapper;
use crate::retro_sys::retro_log_level;
use crate::{
    binding::binding_libretro::retro_game_info,
    erro_handle::{ErroHandle, RetroLogLevel},
};
use std::fs;
use std::io::Write;
use std::{
    ffi::CString,
    fs::File,
    io::Read,
    os::raw::c_void,
    path::{Path, PathBuf},
    ptr::null,
};

fn get_full_path(path: &str) -> Result<PathBuf, ErroHandle> {
    match PathBuf::from(path).canonicalize() {
        Ok(full_path) => Ok(full_path),
        Err(e) => Err(ErroHandle {
            level: RetroLogLevel::RETRO_LOG_ERROR,
            message: e.to_string(),
        }),
    }
}

fn valid_rom_extension(ctx: &CoreWrapper, path: &Path) -> Result<(), ErroHandle> {
    let valid_extensions = ctx.system.info.valid_extensions.read().unwrap();
    let path_str = path.extension().unwrap().to_str().unwrap();

    if !valid_extensions.contains(path_str) {
        return Err(ErroHandle {
            level: RetroLogLevel::RETRO_LOG_ERROR,
            message: "Extensão da rom invalida: valores esperados -> ".to_string()
                + &valid_extensions.to_string()
                + "; valor recebido -> "
                + path_str,
        });
    };

    Ok(())
}

fn get_save_path(ctx: &CoreWrapper, slot: usize) -> Result<PathBuf, ErroHandle> {
    let mut path = PathBuf::from(ctx.paths.save.clone());
    path.push(&*ctx.rom_name.lock().unwrap());

    if !path.exists() {
        fs::create_dir(&path).unwrap();
    }

    path.push(slot.to_string());
    path.set_extension("save");

    Ok(path)
}

pub struct RomTools;

impl RomTools {
    pub fn create_game_info(ctx: &CoreWrapper, path: &str) -> Result<bool, ErroHandle> {
        let f_path = get_full_path(path)?;

        valid_rom_extension(ctx, &f_path)?;

        let mut buf = Vec::new();
        let meta = CString::new("").unwrap();
        let path = make_c_string(&f_path.to_str().unwrap())?;
        let mut size = 0;

        let need_full_path = *ctx.system.info.need_full_path.read().unwrap();

        if !need_full_path {
            let mut file = File::open(&f_path).unwrap();

            size = file.metadata().unwrap().len() as usize;

            buf = Vec::with_capacity(size);

            file.read_to_end(&mut buf).unwrap();
        }

        let game_info = retro_game_info {
            data: if buf.is_empty() {
                null()
            } else {
                buf.as_ptr() as *const c_void
            },
            meta: meta.as_ptr(),
            path: path.as_ptr(),
            size,
        };

        let state = unsafe { ctx.raw.retro_load_game(&game_info) };

        Ok(state)
    }

    pub fn get_rom_name(path: &PathBuf) -> Result<String, ErroHandle> {
        let extension = path.extension().unwrap();
        let name = path
            .file_name()
            .unwrap()
            .to_str()
            .to_owned()
            .unwrap()
            .to_string()
            .replace(extension.to_str().unwrap(), "");

        Ok(name)
    }

    pub fn create_save_state(ctx: &CoreWrapper, slot: usize) -> Result<(), ErroHandle> {
        let size = unsafe { ctx.raw.retro_serialize_size() };
        let mut data = vec![0u8; size];

        let state = unsafe {
            ctx.raw
                .retro_serialize(data.as_mut_ptr() as *mut c_void, size)
        };

        if !state {
            return Err(ErroHandle {
                level: RetroLogLevel::RETRO_LOG_ERROR,
                message: "nao foi possível salva o estado atual".to_string(),
            });
        }

        let mut file = File::create(get_save_path(ctx, slot)?).unwrap();
        file.write(&data).unwrap();

        Ok(())
    }

    pub fn load_save_state(ctx: &CoreWrapper, slot: usize) -> Result<(), ErroHandle> {
        let save_path = get_save_path(ctx, slot)?;

        let mut save_file = File::open(save_path).unwrap();

        let mut buff = Vec::new();
        save_file.read_to_end(&mut buff).unwrap();

        let core_expect_size = unsafe { ctx.raw.retro_serialize_size() };
        let buffer_size = buff.len();

        if buffer_size != core_expect_size {
            return Err(ErroHandle {
                level: retro_log_level::RETRO_LOG_ERROR,
                message: "o state escolhido nao e correspondente ao core".to_string(),
            });
        }

        unsafe {
            let suss = ctx
                .raw
                .retro_unserialize(buff.as_mut_ptr() as *mut c_void, buffer_size);

            if !suss {
                return Err(ErroHandle {
                    level: retro_log_level::RETRO_LOG_ERROR,
                    message: "o core nao pode carregar o state escolhido".to_string(),
                });
            }
        }

        Ok(())
    }
}
