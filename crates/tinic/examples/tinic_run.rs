use generics::test_workdir::{
    create_test_work_dir_path, get_test_core_path, get_test_rom_path, remove_test_work_dir_path,
};
use tinic::{ErrorHandle, Tinic, TinicGameInfo};
mod common;
use common::events::{DeviceEvents, WindowEvents};

fn main() -> Result<(), ErrorHandle> {
    let mut tinic = Tinic::new()?;
    tinic.set_controle_listener(Box::new(DeviceEvents))?;
    tinic.set_window_listener(Box::new(WindowEvents));

    let test_dir = "tinic_example";

    let game_info = TinicGameInfo {
        core: get_test_core_path().display().to_string(),
        rom: get_test_rom_path().display().to_string(),
        sys_dir: create_test_work_dir_path(test_dir).display().to_string(),
    };

    let game_instance = tinic.create_game_instance(game_info)?;

    // isso irá travar a thread atual até que o jogo seja encerrado
    // se precisar de mais controle sobre o loop de jogo, você pode usar Tinic::pop_event
    tinic.run(game_instance)?;

    remove_test_work_dir_path(test_dir)
}
