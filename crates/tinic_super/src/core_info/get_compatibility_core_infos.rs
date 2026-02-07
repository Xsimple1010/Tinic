use crate::core_info::has_core_installed::this_core_is_installed;
use crate::core_info::model::CoreInfo;
use crate::core_info::read_file::read_info_file_blocking;
use generics::retro_paths::RetroPaths;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use std::path::PathBuf;

pub async fn get_compatibility_core_infos(
    rom_path: &PathBuf,
    retro_paths: &RetroPaths,
) -> Vec<CoreInfo> {
    let extension = match rom_path.extension().and_then(|e| e.to_str()) {
        Some(ext) => ext.to_string(),
        None => return Vec::new(),
    };

    let infos_dir = retro_paths.infos.to_string();
    let core_dir = retro_paths.cores.clone();

    let res = tokio::task::spawn_blocking(move || {
        let entries = std::fs::read_dir(infos_dir).ok()?;

        let cores: Vec<CoreInfo> = entries
            .par_bridge()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let mut info = read_info_file_blocking(&entry.path()).ok()?;

                this_core_is_installed(&core_dir, &mut info).ok()?;

                if info.supported_extensions.contains(&extension) {
                    Some(info)
                } else {
                    None
                }
            })
            .collect();

        Some(cores)
    })
    .await;

    res.ok().flatten().unwrap_or_default()
}
