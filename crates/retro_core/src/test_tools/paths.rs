use generics::erro_handle::ErroHandle;
use generics::retro_paths::RetroPaths;

pub fn get_paths() -> Result<RetroPaths, ErroHandle> {
    RetroPaths::new(
        "retro_out_test/system".to_string(),
        "retro_out_test/save".to_string(),
        "retro_out_test/opt".to_string(),
        "retro_out_test/assents".to_string(),
        "retro_out_test/temps".to_string(),
        "retro_out_test/cores".to_string(),
        "retro_out_test/infos".to_string(),
    )
}
