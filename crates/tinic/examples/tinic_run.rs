use crate::common::setup::{TINIC_EXAMPLE_DIR, create_game_instance, create_tinic};
use generics::test_workdir::remove_test_work_dir_path;
use tinic::ErrorHandle;
mod common;

fn main() -> Result<(), ErrorHandle> {
    let mut tinic = create_tinic()?;
    let game_instance = create_game_instance(&mut tinic)?;

    // isso irá travar a thread atual até que o jogo seja encerrado
    // se precisar de mais controle sobre o loop de jogo, você pode usar Tinic::pop_event
    tinic.run(game_instance)?;

    remove_test_work_dir_path(TINIC_EXAMPLE_DIR)
}
