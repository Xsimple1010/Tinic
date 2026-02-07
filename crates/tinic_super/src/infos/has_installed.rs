use std::{fs, path::PathBuf};

use generics::error_handle::ErrorHandle;

pub fn has_installed(dir: PathBuf) -> Result<bool, ErrorHandle> {
    let dir_entry = fs::read_dir(dir)?;

    if dir_entry.count() > 1 {
        Ok(true)
    } else {
        Ok(false)
    }
}
