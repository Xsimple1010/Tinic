use super::ffi_tools::make_c_string;
use crate::system::SysInfo;
use generics::constants::SAVE_EXTENSION_FILE;
use generics::erro_handle::ErroHandle;
use libretro_sys::binding_libretro::{retro_game_info, LibretroRaw};
use std::fs;
use std::io::Write;
use std::sync::Arc;
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
            message: e.to_string(),
        }),
    }
}

fn valid_rom_extension(valid_extensions: &String, path: &Path) -> Result<(), ErroHandle> {
    let path_str = path
        .extension()
        .ok_or(ErroHandle::new(
            "Nao foi possível ler as extensões compatíveis com o core",
        ))?
        .to_str()
        .ok_or(ErroHandle::new(
            "Nao foi possível ler as extensões compatíveis com o core",
        ))?;

    if !valid_extensions.contains(path_str) {
        return Err(ErroHandle {
            message: "Extensão da rom invalida: valores esperados -> ".to_string()
                + &valid_extensions.to_string()
                + "; valor recebido -> "
                + path_str,
        });
    };

    Ok(())
}

fn get_save_path(
    save_dir: &String,
    sys_info: &SysInfo,
    rom_name: &String,
    slot: usize,
) -> Result<PathBuf, ErroHandle> {
    let mut path = PathBuf::from(save_dir);

    path.push(sys_info.library_name.as_str());
    path.push(rom_name);

    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    let file_name = format!("{}.{}", slot, SAVE_EXTENSION_FILE);
    path.push(file_name);

    Ok(path)
}

pub struct RomTools;

impl RomTools {
    pub fn try_load_game(
        libretro_raw: &Arc<LibretroRaw>,
        sys_info: &SysInfo,
        path: &str,
    ) -> Result<bool, ErroHandle> {
        let f_path = get_full_path(path)?;

        valid_rom_extension(&sys_info.valid_extensions, &f_path)?;

        let mut buf = Vec::new();
        let meta = CString::new("")?;
        let path = make_c_string(f_path.to_str().ok_or(ErroHandle::new(
            "nao foi possível transforma o PathBuf da rom para uma string",
        ))?)?;
        let mut size = 0;

        if !*sys_info.need_full_path {
            let mut file = File::open(&f_path)?;

            size = file.metadata()?.len() as usize;

            buf = Vec::with_capacity(size);

            file.read_to_end(&mut buf)?;
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

        let state = unsafe { libretro_raw.retro_load_game(&game_info) };

        Ok(state)
    }

    pub fn get_rom_name(path: &Path) -> Result<String, ErroHandle> {
        let extension = ".".to_owned()
            + path
                .extension()
                .ok_or(ErroHandle::new("erro ao tentar recuperar o nome da rom"))?
                .to_str()
                .ok_or(ErroHandle::new("erro ao tentar recuperar o nome da rom"))?;

        let name = path
            .file_name()
            .ok_or(ErroHandle::new("erro ao tentar recuperar o nome da rom"))?
            .to_str()
            .ok_or(ErroHandle::new("erro ao tentar recuperar o nome da rom"))?
            .replace(&extension, "");

        Ok(name)
    }

    pub fn create_save_state(
        libretro_raw: &Arc<LibretroRaw>,
        save_dir: &String,
        sys_info: &SysInfo,
        rom_name: &String,
        slot: usize,
    ) -> Result<PathBuf, ErroHandle> {
        let size = unsafe { libretro_raw.retro_serialize_size() };
        let mut data = vec![0u8; size];

        let state = unsafe { libretro_raw.retro_serialize(data.as_mut_ptr() as *mut c_void, size) };

        if !state {
            return Err(ErroHandle {
                message: "nao foi possível salva o estado atual".to_string(),
            });
        }

        let save_path = get_save_path(save_dir, sys_info, rom_name, slot)?;

        match File::create(&save_path) {
            Ok(mut file) => {
                if let Err(e) = file.write(&data) {
                    return Err(ErroHandle {
                        message: e.to_string(),
                    });
                }

                Ok(save_path)
            }
            Err(e) => Err(ErroHandle {
                message: e.to_string(),
            }),
        }
    }

    pub fn load_save_state(
        libretro_raw: &Arc<LibretroRaw>,
        save_dir: &String,
        sys_info: &SysInfo,
        rom_name: &String,
        slot: usize,
    ) -> Result<(), ErroHandle> {
        let save_path = get_save_path(save_dir, sys_info, rom_name, slot)?;

        let mut save_file = File::open(save_path)?;

        let mut buff = Vec::new();
        save_file.read_to_end(&mut buff)?;

        let core_expect_size = unsafe { libretro_raw.retro_serialize_size() };
        let buffer_size = buff.len();

        if buffer_size != core_expect_size {
            return Err(ErroHandle {
                message: "o state escolhido nao e correspondente ao core".to_string(),
            });
        }

        unsafe {
            let suss =
                libretro_raw.retro_unserialize(buff.as_mut_ptr() as *mut c_void, buffer_size);

            if !suss {
                return Err(ErroHandle {
                    message: "o core nao pode carregar o state escolhido".to_string(),
                });
            }
        }

        Ok(())
    }
}
