use crate::GameIdentifier;
use crate::event::TinicSuperEventListener;
use crate::rdb_manager::download::download_rdb;
use crate::rdb_manager::rdb_parser::{read_rdb, read_rdb_blocking, read_rdbs};
use generics::error_handle::ErrorHandle;
use generics::retro_paths::RetroPaths;
use std::collections::HashSet;
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

    pub async fn read_rdbs(&self, rdb_names: HashSet<String>) {
        read_rdbs(
            rdb_names,
            self.retro_path.databases.to_string(),
            self.event_listener.clone(),
        )
        .await;
    }

    pub async fn identify_roms_from_dir(
        &self,
        dir: PathBuf,
    ) -> Result<Vec<GameIdentifier>, ErrorHandle> {
        GameIdentifier::from_dir(dir).await
    }

    pub async fn download(&self, force_update: bool) -> Result<(), ErrorHandle> {
        download_rdb(
            self.retro_path.clone(),
            force_update,
            self.event_listener.clone(),
        )
        .await
    }
}
