use generics::test_workdir::{create_test_work_dir_path, get_test_core_path, get_test_rom_path};
use tinic::{ErrorHandle, GameInstance, Tinic, TinicGameInfo};

use crate::common::events::{DeviceEvents, WindowEvents};

pub fn create_tinic() -> Result<Tinic, ErrorHandle> {
    let mut tinic = Tinic::new()?;
    // definir os listerners é obriga
    tinic.set_controle_listener(Box::new(DeviceEvents))?;
    tinic.set_window_listener(Box::new(WindowEvents));
    Ok(tinic)
}

pub const TINIC_EXAMPLE_DIR: &str = "tinic_example";

pub fn create_game_instance(tinic: &mut Tinic) -> Result<GameInstance, ErrorHandle> {
    let game_info = TinicGameInfo {
        core: get_test_core_path().display().to_string(),
        rom: get_test_rom_path().display().to_string(),
        sys_dir: create_test_work_dir_path(TINIC_EXAMPLE_DIR)
            .display()
            .to_string(),
    };

    tinic.create_game_instance(game_info)
}

// adiciona funções auxiliares
