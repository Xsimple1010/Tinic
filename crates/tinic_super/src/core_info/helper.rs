use crate::core_info::model::CoreInfo;
use crate::download::FileProgress;
use crate::download::download_file;
use crate::event::TinicSuperEventListener;
use crate::extract_files::{SevenZipBeforeExtractionAction, extract_7zip_file, extract_zip_file};
use generics::constants::{CORE_INFOS_URL, cores_url};
use generics::error_handle::ErrorHandle;
use generics::retro_paths::RetroPaths;
use rayon::prelude::*;
use std::sync::Arc;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub struct CoreInfoHelper;

impl CoreInfoHelper {
    pub async fn try_update_core_infos(
        retro_paths: &RetroPaths,
        force_update: bool,
        event_listener: Arc<dyn TinicSuperEventListener>,
    ) -> Result<(), ErrorHandle> {
        let temp_dir = PathBuf::from(&retro_paths.temps.to_string());

        let path = download_file(
            CORE_INFOS_URL,
            "info.zip",
            temp_dir.clone(),
            force_update,
            event_listener.clone(),
        )
        .await
        .map_err(|e| ErrorHandle::new(&e.to_string()))?;

        extract_zip_file(
            path,
            retro_paths.infos.clone().to_string(),
            event_listener.clone(),
        )
        .unwrap();

        let core_url = cores_url()?;
        download_file(core_url, "cores.7z", temp_dir, force_update, event_listener)
            .await
            .map_err(|e| ErrorHandle::new(&e.to_string()))?;

        Ok(())
    }

    pub fn read_info_file(file_path: &PathBuf) -> Result<CoreInfo, ErrorHandle> {
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

        info.file_name = file_path
            .file_name()
            .ok_or(ErrorHandle::new("File has no file name"))?
            .to_str()
            .ok_or(ErrorHandle::new("File has no file name"))?
            .to_string()
            .replace(".info", "");

        Ok(info)
    }

    pub fn install_core(retro_paths: &RetroPaths, core_file_name: &Vec<String>) {
        extract_7zip_file(
            format!("{}/cores.7z", &retro_paths.temps).into(),
            retro_paths.cores.to_string(),
            |file_progress: FileProgress| match file_progress {
                FileProgress::Extract(name) => {
                    let name = name.replace(".so", "").replace(".dll", "");
                    if core_file_name.contains(&name) {
                        return SevenZipBeforeExtractionAction::Extract;
                    }
                    SevenZipBeforeExtractionAction::Jump
                }
                FileProgress::Download(_, _) => SevenZipBeforeExtractionAction::Jump,
            },
        );
    }

    pub fn has_core_installed(retro_paths: &RetroPaths) -> bool {
        match std::fs::read_dir(&retro_paths.cores.to_string()) {
            Ok(rd) => {
                if rd.count() > 0 {
                    true
                } else {
                    false
                }
            }
            Err(_) => false,
        }
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

    pub fn get_compatibility_core_infos(
        rom_path: &PathBuf,
        retro_paths: &RetroPaths,
    ) -> Vec<CoreInfo> {
        let extension = match rom_path.extension().and_then(|e| e.to_str()) {
            Some(ext) => ext,
            None => return Vec::new(),
        };

        let entries = match std::fs::read_dir(&retro_paths.infos.to_string()) {
            Ok(rd) => rd,
            Err(_) => return Vec::new(),
        };

        entries
            .par_bridge()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let info = CoreInfoHelper::read_info_file(&entry.path()).ok()?;

                if info.supported_extensions.contains(extension) {
                    Some(info)
                } else {
                    None
                }
            })
            .collect()
    }
}
