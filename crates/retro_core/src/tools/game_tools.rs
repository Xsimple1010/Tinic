use super::ffi_tools::make_c_string;
use crate::system::SysInfo;
use generics::constants::SAVE_EXTENSION_FILE;
use generics::erro_handle::ErroHandle;
use libretro_sys::binding_libretro::{retro_game_info, LibretroRaw};
use std::fs;
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

fn get_save_path(save_info: &SaveInfo) -> Result<PathBuf, ErroHandle> {
    let mut path = PathBuf::from(save_info.save_dir);

    path.push(save_info.library_name);
    path.push(save_info.rom_name);

    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    let file_name = format!("{}.{}", save_info.slot, SAVE_EXTENSION_FILE);
    path.push(file_name);

    Ok(path)
}

pub struct RomTools;

pub struct SaveInfo<'a> {
    pub save_dir: &'a String,
    pub library_name: &'a String,
    pub rom_name: &'a String,
    pub slot: usize,
    pub buffer_size: usize,
}

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

    pub fn create_save_state<CA>(save_info: SaveInfo, get_data: CA) -> Result<PathBuf, ErroHandle>
    where
        CA: FnOnce(&mut Vec<u8>, usize) -> bool,
    {
        let mut data = vec![0u8; save_info.buffer_size];

        let state = get_data(&mut data, save_info.buffer_size);

        if !state {
            return Err(ErroHandle {
                message: "nao foi possível salva o estado atual".to_string(),
            });
        }

        let save_path = get_save_path(&save_info)?;
        fs::write(save_path.clone(), data)?;

        Ok(save_path)
    }

    pub fn load_save_state<CA>(save_info: SaveInfo, send_to_core: CA) -> Result<(), ErroHandle>
    where
        CA: FnOnce(&mut Vec<u8>, usize) -> bool,
    {
        let save_path = get_save_path(&save_info)?;

        if !save_path.exists() {
            println!("O save escolhido nao existe");
            return Ok(());
        }

        let mut data = fs::read(save_path)?;
        let buffer_size = data.len();

        if buffer_size > save_info.buffer_size {
            println!("o state escolhido nao e correspondente ao core");
            return Ok(());
        }

        if !send_to_core(&mut data, buffer_size) {
            println!("o core nao pode carregar o state escolhido");
        }

        Ok(())
    }
}
