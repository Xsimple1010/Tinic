use crate::database::game::Game;
use crate::database::rdb::{parse_all_rdb_to_vec, parse_rdb};
use crate::download::download_file;
use crate::FileProgress;
use generics::constants::RDB_BASE_URL;
use generics::{error_handle::ErrorHandle, retro_paths::RetroPaths};
use std::path::{Path, PathBuf};

pub struct DatabaseHelper {
    pub rdb_file: String,
}

#[derive(Debug)]
pub struct RDBDatabase {
    pub name: String,
    pub file: PathBuf,
}

impl DatabaseHelper {
    pub fn get_all_games(self) -> Result<Vec<Game>, ErrorHandle> {
        parse_all_rdb_to_vec(&self.rdb_file)
    }

    pub fn search_by_crc(&self, src: u32) -> Result<Option<Game>, ErrorHandle> {
        let mut out_game = None;

        let _ = parse_rdb(&self.rdb_file, |game| {
            if game.crc32 == Some(src) {
                out_game = Some(game);

                true
            } else {
                false
            }
        });

        Ok(out_game)
    }

    pub fn search_by_name(&self, name: &String) -> Result<Vec<Game>, ErrorHandle> {
        let mut out_game: Vec<Game> = Vec::new();

        parse_rdb(&self.rdb_file, |game| {
            // println!("{game:?}");

            let game_name = match &game.name {
                Some(name) => name,
                None => return false,
            };

            if game_name.to_lowercase().contains(name) {
                out_game.push(game);
            }

            false
        })?;

        Ok(out_game)
    }

    pub fn get_installed_rdb(paths: &RetroPaths) -> Result<Vec<RDBDatabase>, ErrorHandle> {
        let database_dir = paths.databases.to_string();

        let read_dir = std::fs::read_dir(database_dir)?;

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
