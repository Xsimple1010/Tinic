use crate::cores::installed::this_core_is_installed;
use crate::infos::model::CoreInfo;
use crate::infos::read_file::read_info_file_blocking;
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

                let is_installed = this_core_is_installed(&core_dir, &mut info.file_name).ok()?;
                info.is_installed = is_installed;

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

    match res.ok().flatten() {
        Some(mut cores) => {
            cores.sort_by_key(|info| info.core_name.clone());
            cores
        }
        None => Vec::with_capacity(0),
    }
}
