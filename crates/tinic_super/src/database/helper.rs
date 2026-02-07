use crate::{
    FileProgress,
    database::{game::Game, rdb::parse_rdb},
    download::download_file,
};
use generics::{constants::RDB_BASE_URL, error_handle::ErrorHandle, retro_paths::RetroPaths};
use std::path::{Path, PathBuf};

pub struct DatabaseHelper {
    games: Vec<Game>,
}

#[derive(Debug)]
pub struct RDBDatabase {
    pub name: String,
    pub file: PathBuf,
}

impl DatabaseHelper {
    pub fn new(rdb_file: String) -> Result<Self, ErrorHandle> {
        Ok(Self {
            games: parse_rdb(rdb_file)?,
        })
    }

    pub fn get_all_games(self) -> Vec<Game> {
        self.games
    }

    pub fn search_by_crc(&self, src: u32) -> Option<&Game> {
        for game in &self.games {
            if game.crc32.eq(&Some(src)) {
                return Some(game);
            }
        }

        None
    }

    pub fn search_by_name(&self, name: String) -> Option<&Game> {
        for game in &self.games {
            let c_name = match &game.name {
                Some(name) => name,
                None => return None,
            };

            if name.contains(c_name) {
                return Some(game);
            }
        }

        None
    }

    pub fn get_instaled_rdb(paths: &RetroPaths) -> Result<Vec<RDBDatabase>, ErrorHandle> {
        let database_dir = paths.databases.to_string();

        let read_dir = std::fs::read_dir(database_dir)?;

        let mut out: Vec<RDBDatabase> = Vec::new();

        for dir_entry in read_dir {
            let entry = dir_entry?;

            let name = entry
                .file_name()
                .into_string()
                .map_err(|_| ErrorHandle::new(&format!("cant create a String from: OsString")))?;

            out.push(RDBDatabase {
                name,
                file: entry.path(),
            });
        }

        Ok(out)
    }

    pub async fn download_db<CP>(
        paths: &RetroPaths,
        rdb_name: &str,
        force_update: bool,
        on_progress: CP,
    ) -> Result<(), ErrorHandle>
    where
        CP: Fn(FileProgress) + Copy,
    {
        if rdb_name.is_empty() {
            return Err(ErrorHandle::new("rdb_name is empty"));
        }

        let mut dbs: Vec<String> = Vec::new();

        for db in rdb_name.split("|") {
            if !db.ends_with(".rdb") {
                dbs.push(format!("{db}.rdb"));
            }
        }

        for rdb_name in dbs {
            let url = format!("{RDB_BASE_URL}/{rdb_name}");

            download_file(
                &url,
                &rdb_name,
                &paths.temps,
                force_update,
                on_progress,
                |temp_path| {
                    let db_dir = PathBuf::from(paths.databases.to_string());

                    let final_path = db_dir.join(
                        Path::new(&temp_path)
                            .file_name()
                            .ok_or_else(|| ErrorHandle::new("invalid temp file name"))
                            .unwrap(),
                    );

                    let _ = std::fs::copy(&temp_path, &final_path)
                        .and_then(|_| std::fs::remove_file(&temp_path))
                        .map_err(|e| ErrorHandle::new(&e.to_string()));
                },
            )
            .await
            .map_err(|e| ErrorHandle::new(&e.to_string()))?;
        }

        Ok(())
    }
}
