use generics::test_workdir::remove_test_work_dir_path;
use tinic::{ErrorHandle, TinicGameInstanceStatus};
mod common;
use crate::common::setup::{TINIC_EXAMPLE_DIR, create_game_instance, create_tinic};

fn main() -> Result<(), ErrorHandle> {
    let mut tinic = create_tinic()?;

    // Aqui você tem controle total sobre o loop de jogo, mas você não pode criar outro game_instance
    // se precisar criar várias games instances uma após a outra use Tinic::run_app_on_demand
    let mut game_instance = create_game_instance(&mut tinic)?;
    loop {
        let status = tinic.pop_event(&mut game_instance);

        if let TinicGameInstanceStatus::Exit(_) = status {
            break;
        }
    }

    remove_test_work_dir_path(TINIC_EXAMPLE_DIR)
}
