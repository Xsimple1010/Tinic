use crate::download::download_file;
use crate::event::TinicSuperEventListener;
use crate::rdb_manager::game::GameInfo;
use crate::rdb_manager::rdb::read_to_end_of_rdb;
use generics::constants::RDB_BASE_URL;
use generics::{error_handle::ErrorHandle, retro_paths::RetroPaths};
use std::path::PathBuf;
use std::sync::Arc;

pub struct RdbManager {
    pub rdb_file: String,
}

#[derive(Debug, Clone)]
pub struct RDBDatabase {
    pub name: String,
    pub file: PathBuf,
}

impl RdbManager {
    pub fn get_installed(rdb_path: &String) -> Result<Vec<RDBDatabase>, ErrorHandle> {
        let read_dir = std::fs::read_dir(rdb_path)?;

        let mut out: Vec<RDBDatabase> = Vec::new();

        for dir_entry in read_dir {
            let entry = dir_entry?;

            let name = entry.file_name().into_string().map_err(|_| {
                ErrorHandle::new(&"cant create a String from: OsString".to_string())
            })?;

            out.push(RDBDatabase {
                name,
                file: entry.path(),
            });
        }

        Ok(out)
    }

    pub fn read_to_end<C: FnMut(Vec<GameInfo>)>(
        rdb_path: &String,
        callback: C,
    ) -> Result<(), ErrorHandle> {
        read_to_end_of_rdb(rdb_path, callback)
    }

    pub async fn download(
        paths: &RetroPaths,
        rdbs: &Vec<String>,
        force_update: bool,
        event_listener: Arc<dyn TinicSuperEventListener>,
    ) -> Result<(), ErrorHandle> {
        if rdbs.is_empty() {
            return Err(ErrorHandle::new("dbs is empty"));
        }

        let mut dbs: Vec<String> = Vec::new();
        for rdb in rdbs {
            if !rdb.ends_with(".rdb") {
                dbs.push(format!("{rdb}.rdb"));
            }
        }

        for rdb_name in dbs {
            let rdb_path = PathBuf::from(paths.databases.to_string()).join(rdb_name.clone());

            if rdb_path.exists() {
                continue;
            }

            let url = format!("{RDB_BASE_URL}/{rdb_name}");
            let databases_dir = PathBuf::from(paths.databases.to_string());
            download_file(
                &url,
                &rdb_name,
                databases_dir,
                force_update,
                event_listener.clone(),
            )
            .await
            .map_err(|e| ErrorHandle::new(&e.to_string()))?;
        }

        Ok(())
    }
}
