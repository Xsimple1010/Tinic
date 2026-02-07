use crate::erro_handle::ErroHandle;
use std::fs;
use std::ops::Not;
use std::path::Path;
use std::sync::Arc;

#[derive(Clone, Debug, Eq)]
pub struct RetroPaths {
    pub system: Arc<String>,
    pub save: Arc<String>,
    pub opt: Arc<String>,
    pub assets: Arc<String>,
    pub temps: Arc<String>,
    pub cores: Arc<String>,
    pub infos: Arc<String>,
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
        temps: String,
        cores: String,
        infos: String,
    ) -> Result<Self, ErroHandle> {
        if Path::new(&system).exists().not() && fs::create_dir_all(&system).is_err() {
            return Err(ErroHandle {
                message: "Não foi possível criar a pasta system".to_owned(),
            });
        }

        if Path::new(&save).exists().not() && fs::create_dir_all(&save).is_err() {
            return Err(ErroHandle {
                message: "Não foi possível criar a pasta save".to_owned(),
            });
        }

        if Path::new(&opt).exists().not() && fs::create_dir_all(&opt).is_err() {
            return Err(ErroHandle {
                message: "Não foi possível criar a pasta opt".to_owned(),
            });
        }

        if Path::new(&assets).exists().not() && fs::create_dir_all(&assets).is_err() {
            return Err(ErroHandle {
                message: "Não foi possível criar a pasta assets".to_owned(),
            });
        }

        if Path::new(&temps).exists().not() && fs::create_dir_all(&temps).is_err() {
            return Err(ErroHandle {
                message: "Não foi possível criar a pasta temps".to_owned(),
            });
        }

        if Path::new(&cores).exists().not() && fs::create_dir_all(&cores).is_err() {
            return Err(ErroHandle {
                message: "Não foi possível criar a pasta cores".to_owned(),
            });
        }

        if Path::new(&infos).exists().not() && fs::create_dir_all(&infos).is_err() {
            return Err(ErroHandle {
                message: "Não foi possível criar a pasta infos".to_owned(),
            });
        }

        Ok(Self {
            system: Arc::new(system),
            opt: Arc::new(opt),
            save: Arc::new(save),
            assets: Arc::new(assets),
            temps: Arc::new(temps),
            cores: Arc::new(cores),
            infos: Arc::new(infos),
        })
    }
}
