use crate::core_info::model::CoreInfo;
use generics::{error_handle::ErrorHandle, retro_paths::RetroPaths};
use std::sync::Arc;

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

pub fn this_core_is_installed(
    cores_dir: &Arc<String>,
    core_info: &mut CoreInfo,
) -> Result<(), ErrorHandle> {
    let entries = std::fs::read_dir(cores_dir.to_string())?;

    for dir_entry in entries {
        let entry = dir_entry?;

        let file_name = match entry.file_name().into_string() {
            Ok(e) => e,
            Err(_) => continue,
        };

        if entry.metadata()?.is_file() && file_name.contains(&core_info.file_name) {
            core_info.is_installed = true;
            break;
        }
    }

    Ok(())
}
