use crate::{
    FileProgress,
    event::TinicSuperEventListener,
    tools::extract_files::{SevenZipBeforeExtractionAction, extract_7zip_file},
};
use generics::{error_handle::ErrorHandle, retro_paths::RetroPaths};
use std::{collections::HashSet, path::PathBuf, sync::Arc};

fn remove_so_extension(name: String) -> String {
    name.replace(".so", "").replace(".dll", "")
}

pub async fn install_core(
    retro_paths: RetroPaths,
    core_file_name: Vec<String>,
    event_listener: Arc<dyn TinicSuperEventListener>,
) {
    let src_path = format!("{}/cores.7z", &retro_paths.temps);

    let mut wanted: HashSet<String> = core_file_name
        .into_iter()
        .map(remove_so_extension)
        .collect();

    tokio::task::spawn_blocking(move || {
        extract_7zip_file(
            src_path.into(),
            retro_paths.cores.to_string(),
            event_listener,
            |file_progress: FileProgress| match file_progress {
                FileProgress::Extract(name) => {
                    let name = remove_so_extension(name);

                    if wanted.remove(&name) {
                        return SevenZipBeforeExtractionAction::Extract;
                    }

                    if wanted.is_empty() {
                        SevenZipBeforeExtractionAction::Stop
                    } else {
                        SevenZipBeforeExtractionAction::Jump
                    }
                }
                FileProgress::Download(_, _) => SevenZipBeforeExtractionAction::Jump,
            },
        );
    });
}

pub fn this_core_is_installed(
    cores_dir: &Arc<String>,
    info_to_search: &String,
) -> Result<bool, ErrorHandle> {
    let entries = std::fs::read_dir(cores_dir.to_string())?;

    for dir_entry in entries {
        let entry = dir_entry?;

        let file_name = match entry.file_name().into_string() {
            Ok(e) => e,
            Err(_) => continue,
        };

        if entry.metadata()?.is_file() && file_name.contains(info_to_search) {
            return Ok(true);
        }
    }

    Ok(false)
}

pub async fn get_installed_cores(cores_dir: &String) -> Result<HashSet<String>, ErrorHandle> {
    let mut out: HashSet<String> = HashSet::new();

    let cores_dir = PathBuf::from(&cores_dir);
    if !cores_dir.exists() {
        tokio::fs::create_dir_all(&cores_dir).await?;
    }

    let mut dir_entry = tokio::fs::read_dir(&cores_dir).await?;

    while let Some(dir_entry) = dir_entry.next_entry().await? {
        if dir_entry.metadata().await?.is_dir() {
            continue;
        }

        let file_name = match dir_entry.file_name().into_string() {
            Ok(e) => e,
            Err(_) => continue,
        };

        out.insert(file_name);
    }

    Ok(out)
}
