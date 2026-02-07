use generics::retro_paths::RetroPaths;

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
