use tinic_generics::test_workdir::remove_test_work_dir_path;
use tinic::ErrorHandle;
mod common;

use crate::common::setup::{TINIC_EXAMPLE_DIR, create_game_instance, create_tinic};

fn main() -> Result<(), ErrorHandle> {
    let mut tinic = create_tinic()?;

    // run_app_on_demand blocks the current thread, but unlike run & pop_event,
    // it allows creating multiple game instances one after another.
    let game_instance = create_game_instance(&mut tinic)?;
    let _status = tinic.run_app_on_demand(game_instance);

    // Right after run_app_on_demand finishes, we can create another game_instance
    //
    // Uncomment this part of the code, and a new window will be created when the first one is closed
    // let game_instance = create_game_instance(&mut tinic)?;
    // let _status = tinic.run_app_on_demand(game_instance);

    remove_test_work_dir_path(TINIC_EXAMPLE_DIR)
}
