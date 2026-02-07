use crate::core_info::model::CoreInfo;
use crate::database::crc32::crc32_file;
use crate::database::game::GameInfo;
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
    pub fn get_all_games(self) -> Result<Vec<GameInfo>, ErrorHandle> {
        parse_all_rdb_to_vec(&self.rdb_file)
    }

    pub fn identifier_rom_file(
        rom_file: &str,
        core_info: &CoreInfo,
        database_dir: &String,
    ) -> Result<Option<GameInfo>, ErrorHandle> {
        let rbs: Vec<RDBDatabase> = Self::get_installed_rdb(database_dir)?
            .into_iter()
            .filter(|rdb| {
                core_info
                    .database
                    .contains(&rdb.name.clone().replace(".rdb", ""))
            })
            .collect();

        let src32 = crc32_file(rom_file)?;
        let mut out_game = None;

        for rdb in rbs {
            let rdb_file = rdb
                .file
                .to_str()
                .ok_or_else(|| ErrorHandle::new("rdb file no exist"))?;

            let _ = parse_rdb(rdb_file, |game| {
                if game.crc32 == Some(src32) {
                    out_game = Some(game);
                    true
                } else {
                    false
                }
            });

            if out_game.is_some() {
                break;
            }
        }

        Ok(out_game)
    }

    pub fn search_by_name(&self, name: &str) -> Result<Vec<GameInfo>, ErrorHandle> {
        let mut out_game: Vec<GameInfo> = Vec::new();

        parse_rdb(&self.rdb_file, |game| {
            // println!("{game:?}");

            let game_name = match &game.name {
                Some(name) => name,
                None => return false,
            };

            let name = name.to_lowercase();
            if game_name.to_lowercase().contains(&name) {
                out_game.push(game);
            }

            false
        })?;

        Ok(out_game)
    }

    pub fn get_installed_rdb(database_dir: &String) -> Result<Vec<RDBDatabase>, ErrorHandle> {
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
        rdbs: &Vec<String>,
        force_update: bool,
        on_progress: CP,
    ) -> Result<(), ErrorHandle>
    where
        CP: Fn(FileProgress) + Copy,
    {
        if rdbs.is_empty() {
            return Err(ErrorHandle::new("dbs is empty"));
        }

        let mut dbs: Vec<String> = Vec::new();
        //
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
