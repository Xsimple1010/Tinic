use generics::error_handle::ErrorHandle;
use generics::retro_paths::RetroPaths;

pub fn get_paths() -> Result<RetroPaths, ErrorHandle> {
    RetroPaths::new(
        "retro_out_test/system".to_string(),
        "retro_out_test/save".to_string(),
        "retro_out_test/opt".to_string(),
        "retro_out_test/assents".to_string(),
        "retro_out_test/temps".to_string(),
        "retro_out_test/cores".to_string(),
        "retro_out_test/infos".to_string(),
        "retro_out_test/databases".to_string(),
    )
}
