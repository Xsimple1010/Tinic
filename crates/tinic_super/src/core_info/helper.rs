use crate::core_info::model::CoreInfo;
use crate::download::FileProgress;
use crate::{download::download_file, extract_files::extract_7zip_file};
use generics::constants::CORE_INFOS_URL;
use generics::{constants::CORES_URL, retro_paths::RetroPaths};
use reqwest::Error;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};
use crate::extract_files::extract_zip_file;

pub struct CoreInfoHelper;

impl CoreInfoHelper {
    pub async fn try_update_core_infos<CP>(
        retro_paths: &RetroPaths,
        force_update: bool,
        on_progress: CP,
    ) -> Result<(), Error>
    where
        CP: Fn(FileProgress) + Copy,
    {
        let temp_dir = retro_paths.temps.clone().to_string();

        download_file(
            CORE_INFOS_URL,
            "info.zip",
            &temp_dir,
            force_update,
            on_progress,
            |infos| {
                extract_zip_file(infos, retro_paths.infos.clone().to_string(), on_progress)
                    .unwrap();
            },
        )
        .await?;

        download_file(
            CORES_URL,
            "cores.7z",
            &temp_dir,
            force_update,
            on_progress,
            |path_buf| {
                extract_7zip_file(path_buf, retro_paths.cores.clone().to_string(), on_progress);
            },
        )
        .await?;

        Ok(())
    }

    pub fn read_info_file(file_path: &PathBuf) -> Result<CoreInfo, io::Error> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        let mut info = CoreInfo::default();

        while let Some(Ok(line)) = lines.next() {
            if let Some((key, value)) = line.split_once('=') {
                info.set_value(
                    key.trim(),
                    value
                        .trim_matches('"')
                        .replacen(" ", "", 1)
                        .replacen('\"', "", 1)
                        .to_string(),
                );
            }
        }

        Ok(info)
    }

    pub fn get_core_infos(dir: &String) -> Vec<CoreInfo> {
        let path = PathBuf::from(dir);

        let mut read_dir = path.read_dir().unwrap();

        let mut infos = Vec::new();

        while let Some(Ok(entry)) = read_dir.next() {
            match CoreInfoHelper::read_info_file(&entry.path()) {
                Ok(info) => infos.push(info),
                Err(_) => continue,
            };
        }

        infos
    }

    pub fn get_compatibility_core_infos(rom_path: PathBuf) -> Vec<CoreInfo> {
        let mut read_dir = rom_path.read_dir().unwrap();

        let mut infos = Vec::new();

        while let Some(Ok(entry)) = read_dir.next() {
            match CoreInfoHelper::read_info_file(&entry.path()) {
                Ok(info) => {
                    let extension = rom_path
                        .extension()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .replace(".", "");

                    if info.supported_extensions.contains(&extension) {
                        infos.push(info);
                    };
                }
                Err(_) => continue,
            };
        }

        infos
    }
}
