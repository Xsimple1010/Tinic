use crate::common::setup::{TINIC_EXAMPLE_DIR, create_game_instance, create_tinic};
use tinic_generics::test_workdir::remove_test_work_dir_path;
use tinic::ErrorHandle;
mod common;

fn main() -> Result<(), ErrorHandle> {
    let mut tinic = create_tinic()?;
    let game_instance = create_game_instance(&mut tinic)?;

    // This will block the current thread until the game is closed
    // If you need more control over the game loop, you can use Tinic::pop_event
    tinic.run(game_instance)?;

    remove_test_work_dir_path(TINIC_EXAMPLE_DIR)
}
