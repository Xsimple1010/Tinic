use crate::core_info::model::CoreInfo;
use crate::download::download_file;
use crate::event::TinicSuperEventListener;
use crate::rdb_manager::crc32::crc32_file;
use crate::rdb_manager::game::GameInfo;
use crate::rdb_manager::rdb::{parse_all_rdb_to_vec, parse_rdb};
use generics::constants::RDB_BASE_URL;
use generics::{error_handle::ErrorHandle, retro_paths::RetroPaths};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::fs::File;
use std::path::{Path, PathBuf};
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
    pub fn get_all_games(self) -> Result<Vec<GameInfo>, ErrorHandle> {
        parse_all_rdb_to_vec(&self.rdb_file)
    }

    pub fn identifier_rom_file_with_any_rdb(
        rom_file: &str,
        core_info: &CoreInfo,
        database_dir: &String,
    ) -> Result<Option<(GameInfo, RDBDatabase)>, ErrorHandle> {
        let rbs: Vec<RDBDatabase> = Self::get_installed_rdb(database_dir)?
            .into_iter()
            .filter(|rdb| {
                core_info
                    .database
                    .contains(&rdb.name.clone().replace(".rdb", ""))
            })
            .collect();

        //metadata
        let rom_path = Path::new(rom_file);

        // CRC32 (abre o arquivo internamente)
        let crc32 = crc32_file(rom_path)?;

        // abre o arquivo UMA vez
        let file = File::open(rom_path)?;

        // metadata
        let rom_size = file.metadata()?.len();
        let rom_extension = rom_path
            .extension()
            .ok_or_else(|| ErrorHandle::new("invalid rom name"))?
            .to_str()
            .ok_or_else(|| ErrorHandle::new("invalid rom name"))?;

        let rom_name = rom_path
            .file_name()
            .ok_or_else(|| ErrorHandle::new("invalid rom extension"))?
            .to_str()
            .ok_or_else(|| ErrorHandle::new("invalid rom extension"))?
            .replace(&format!(".{}", rom_extension), "");

        let out_game = rbs
            .par_iter()
            .find_map_any(|rdb| Self::identifier_rom_file(rom_size, &rom_name, crc32, rdb));

        Ok(out_game)
    }

    pub fn identifier_rom_file(
        rom_size: u64,
        rom_name: &str,
        crc32: u32,
        rdb: &RDBDatabase,
    ) -> Option<(GameInfo, RDBDatabase)> {
        let mut out_game = None;
        let rdb_file = match rdb.file.to_str() {
            Some(file) => file,
            None => return None,
        };

        let _ = parse_rdb(rdb_file, |game| {
            if game.crc32 == Some(crc32) {
                out_game = Some(game);
                return true;
            }

            if let Some(name) = &game.name {
                if let Some(size) = game.size {
                    if name.eq(rom_name) && size == rom_size {
                        out_game = Some(game);
                        return true;
                    }

                    // para possivel rom modificada
                    if name.eq(rom_name) {
                        out_game = Some(game);
                        return true;
                    }
                }
            }

            false
        });

        match out_game {
            Some(game) => Some((game, rdb.clone())),
            None => None,
        }
    }

    pub fn search_by_name(&self, name: &str) -> Result<Vec<GameInfo>, ErrorHandle> {
        let mut out_game: Vec<GameInfo> = Vec::new();

        parse_rdb(&self.rdb_file, |game| {
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

    pub async fn download_db(
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
