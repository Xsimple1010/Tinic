use crate::erro_handle::ErroHandle;
use libretro_sys::binding_libretro::retro_log_level::RETRO_LOG_ERROR;
use std::fs;
use std::ops::Not;
use std::path::Path;

#[derive(Clone, Debug, Eq)]
pub struct RetroPaths {
    pub system: String,
    pub save: String,
    pub opt: String,
    pub assets: String,
}

impl PartialEq for RetroPaths {
    fn eq(&self, other: &Self) -> bool {
        other.assets == self.assets && other.system == self.system
    }
}

impl RetroPaths {
    pub fn new(
        system: String,
        save: String,
        opt: String,
        assets: String,
    ) -> Result<Self, ErroHandle> {
        if Path::new(&system).exists().not() && fs::create_dir_all(&system).is_err() {
            return Err(ErroHandle {
                level: RETRO_LOG_ERROR,
                message: "Não foi possível criar a pasta system".to_owned(),
            });
        }

        if Path::new(&save).exists().not() && fs::create_dir_all(&save).is_err() {
            return Err(ErroHandle {
                level: RETRO_LOG_ERROR,
                message: "Não foi possível criar a pasta save".to_owned(),
            });
        }

        if Path::new(&opt).exists().not() && fs::create_dir_all(&opt).is_err() {
            return Err(ErroHandle {
                level: RETRO_LOG_ERROR,
                message: "Não foi possível criar a pasta opt".to_owned(),
            });
        }

        if Path::new(&assets).exists().not() && fs::create_dir_all(&assets).is_err() {
            return Err(ErroHandle {
                level: RETRO_LOG_ERROR,
                message: "Não foi possível criar a pasta assets".to_owned(),
            });
        }

        Ok(Self {
            system,
            opt,
            save,
            assets,
        })
    }
}
