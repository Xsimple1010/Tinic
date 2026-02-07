use crate::error_handle::ErrorHandle;
use std::fmt::Display;
use std::fs;
use std::ops::Not;
use std::path::Path;
use std::sync::Arc;

#[derive(Clone, Debug, Eq)]
pub struct RetroPaths {
    pub base_dir: Arc<String>,
    pub system: Arc<String>,
    pub save: Arc<String>,
    pub opt: Arc<String>,
    pub assets: Arc<String>,
    pub temps: Arc<String>,
    pub cores: Arc<String>,
    pub infos: Arc<String>,
    pub databases: Arc<String>,
    pub arts: Arc<String>,
}

impl PartialEq for RetroPaths {
    fn eq(&self, other: &Self) -> bool {
        other.assets == self.assets && other.system == self.system
    }
}

impl RetroPaths {
    fn new(
        base_dir: String,
        system: String,
        save: String,
        opt: String,
        assets: String,
        temps: String,
        cores: String,
        infos: String,
        databases: String,
        arts: String,
    ) -> Result<Self, ErrorHandle> {
        if Path::new(&system).exists().not() && fs::create_dir_all(&system).is_err() {
            return Err(ErrorHandle {
                message: "Não foi possível criar a pasta system".to_owned(),
            });
        }

        if Path::new(&save).exists().not() && fs::create_dir_all(&save).is_err() {
            return Err(ErrorHandle {
                message: "Não foi possível criar a pasta save".to_owned(),
            });
        }

        if Path::new(&opt).exists().not() && fs::create_dir_all(&opt).is_err() {
            return Err(ErrorHandle {
                message: "Não foi possível criar a pasta opt".to_owned(),
            });
        }

        if Path::new(&assets).exists().not() && fs::create_dir_all(&assets).is_err() {
            return Err(ErrorHandle {
                message: "Não foi possível criar a pasta assets".to_owned(),
            });
        }

        if Path::new(&temps).exists().not() && fs::create_dir_all(&temps).is_err() {
            return Err(ErrorHandle {
                message: "Não foi possível criar a pasta temps".to_owned(),
            });
        }

        if Path::new(&cores).exists().not() && fs::create_dir_all(&cores).is_err() {
            return Err(ErrorHandle {
                message: "Não foi possível criar a pasta cores".to_owned(),
            });
        }

        if Path::new(&infos).exists().not() && fs::create_dir_all(&infos).is_err() {
            return Err(ErrorHandle {
                message: "Não foi possível criar a pasta infos".to_owned(),
            });
        }

        if Path::new(&databases).exists().not() && fs::create_dir_all(&databases).is_err() {
            return Err(ErrorHandle {
                message: "Não foi possível criar a pasta databases".to_owned(),
            });
        }

        if Path::new(&arts).exists().not() && fs::create_dir_all(&arts).is_err() {
            return Err(ErrorHandle {
                message: "Não foi possível criar a pasta arts".to_owned(),
            });
        }

        Ok(Self {
            base_dir: Arc::new(base_dir),
            system: Arc::new(system),
            opt: Arc::new(opt),
            save: Arc::new(save),
            assets: Arc::new(assets),
            temps: Arc::new(temps),
            cores: Arc::new(cores),
            infos: Arc::new(infos),
            databases: Arc::new(databases),
            arts: Arc::new(arts),
        })
    }

    pub fn from_base(base: impl Display) -> Result<Self, ErrorHandle> {
        let sys = format!("{}/system", base);
        let save = format!("{}/save", base);
        let opt = format!("{}/opt", base);
        let assets = format!("{}/assets", base);
        let temps = format!("{}/temps", base);
        let cores = format!("{}/cores", base);
        let infos = format!("{}/infos", base);
        let databases = format!("{}/databases", base);
        let arts = format!("{}/arts", base);

        Self::new(
            base.to_string(),
            sys,
            save,
            opt,
            assets,
            temps,
            cores,
            infos,
            databases,
            arts,
        )
    }
}
