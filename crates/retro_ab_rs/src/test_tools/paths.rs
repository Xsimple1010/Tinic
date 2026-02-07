use crate::erro_handle::ErroHandle;
use crate::paths::Paths;

pub fn get_paths() -> Result<Paths, ErroHandle> {
    Paths::new(
        "retro_out_test/system".to_string(),
        "retro_out_test/save".to_string(),
        "retro_out_test/opt".to_string(),
        "retro_out_test/assents".to_string(),
    )
}
