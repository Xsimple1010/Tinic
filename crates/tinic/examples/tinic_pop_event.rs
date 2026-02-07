use generics::test_workdir::{
    create_test_work_dir_path, get_test_core_path, get_test_rom_path, remove_test_work_dir_path,
};
use tinic::{ErrorHandle, Tinic, TinicGameInfo, TinicGameInstanceStatus};
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

    // Aqui você tem controle total sobre o loop de jogo, mas você não pode criar outro game_instance
    // se precisar criar várias games instances uma após a outra use Tinic::run_app_on_demand
    let mut game_instance = tinic.create_game_instance(game_info)?;
    loop {
        let status = tinic.pop_event(&mut game_instance);

        if let TinicGameInstanceStatus::Exit(_) = status {
            break;
        }
    }

    remove_test_work_dir_path(test_dir)
}
