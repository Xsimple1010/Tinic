use crate::core_info::model::CoreInfo;
use crate::event::TinicSuperEventListener;
use crate::rdb_manager::download::download_rdb;
use crate::rdb_manager::rdb_parser::{read_rdb, read_rdb_blocking, read_rdb_from_cores};
use generics::error_handle::ErrorHandle;
use generics::retro_paths::RetroPaths;
use std::path::PathBuf;
use std::sync::Arc;

pub struct RdbManager {
    pub retro_path: RetroPaths,
    pub event_listener: Arc<dyn TinicSuperEventListener>,
}

#[derive(Debug, Clone)]
pub struct RDBDatabase {
    pub name: String,
    pub file: PathBuf,
}

impl RdbManager {
    pub fn read_rdb_blocking(&self, rdb_path: &String) -> Result<(), ErrorHandle> {
        read_rdb_blocking(rdb_path, self.event_listener.clone())
    }

    pub async fn read_rb(&self, rdb_path: String) {
        read_rdb(rdb_path, self.event_listener.clone()).await;
    }

    pub async fn read_rdb_from_cores(&self, cores: Vec<CoreInfo>) {
        read_rdb_from_cores(
            cores,
            self.retro_path.databases.to_string(),
            self.event_listener.clone(),
        )
        .await;
    }

    pub async fn download(
        &self,
        paths: &RetroPaths,
        rdbs: &Vec<String>,
        force_update: bool,
    ) -> Result<(), ErrorHandle> {
        download_rdb(paths, rdbs, force_update, self.event_listener.clone()).await
    }
}
