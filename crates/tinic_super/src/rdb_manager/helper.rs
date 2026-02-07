use crate::event::TinicSuperEventListener;
use crate::rdb_manager::download::download_rdb;
use crate::rdb_manager::game_model::GameInfo;
use crate::rdb_manager::rdb_parser::read_rdbs_from_dir;
use crate::tools::extract_files::ExtractProgress;
use crate::{DownloadProgress, GameIdentifier};
use generics::error_handle::ErrorHandle;
use generics::retro_paths::RetroPaths;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug)]
pub enum RdbEventType {
    Downloading(DownloadProgress),
    Extracting(ExtractProgress),
    Reading { game_infos: Vec<GameInfo> },
    StartRead { name: String },
    ReadCompleted { remaining: usize, name: String },
}

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
    pub fn read_rdbs(&self) -> Result<(), ErrorHandle> {
        read_rdbs_from_dir(
            &self.retro_path.databases.to_string().into(),
            self.event_listener.clone(),
        )
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
