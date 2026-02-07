use generics::error_handle::ErrorHandle;
use generics::retro_paths::RetroPaths;

pub fn get_paths() -> Result<RetroPaths, ErrorHandle> {
    RetroPaths::from_base("retro_out_test".to_string())
}
