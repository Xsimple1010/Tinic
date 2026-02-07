use generics::test_workdir::remove_test_work_dir_path;
use tinic::ErrorHandle;
mod common;

use crate::common::setup::{TINIC_EXAMPLE_DIR, create_game_instance, create_tinic};

fn main() -> Result<(), ErrorHandle> {
    let mut tinic = create_tinic()?;

    // run_app_on_demand trava a thread atual, mas diferente de run & pop_event
    // ele permite criar varias games instances uma após a outra.
    let game_instance = create_game_instance(&mut tinic)?;
    let _status = tinic.run_app_on_demand(game_instance);

    //
    // logo após a execução do run_app_on_demand, podemos criar outro game_instance
    //
    // removar o commit dessa parte do codigo e uma nova janela será criada quando a primeira for fechada
    // let game_instance = create_game_instance(&mut tinic)?;
    // let _status = tinic.run_app_on_demand(game_instance);

    remove_test_work_dir_path(TINIC_EXAMPLE_DIR)
}
