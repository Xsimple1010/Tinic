use crate::app_state::AppStateHandle;
use crate::io::stdout_writer::StdoutWriter;
use std::sync::atomic::Ordering;
use std::thread::sleep;
use std::time::Duration;
use tinic::{ErrorHandle, Tinic};
use crate::constants::THREAD_SLEEP_TIME_IN_MILLISECONDS;

pub fn game_loop(app_state: AppStateHandle, mut tinic: Tinic) -> Result<(), ErrorHandle> {
    loop {
        sleep(Duration::from_millis(THREAD_SLEEP_TIME_IN_MILLISECONDS));

        if !app_state.running.load(Ordering::SeqCst) {
            break;
        }

        let mut game_info = match app_state.game_info.lock() {
            Ok(game_info) => game_info,
            Err(_) => {
                continue;
            }
        };

        if let Some(game_info) = game_info.take() {
            let game_instance = tinic.create_game_instance(game_info)?;

            tinic.run_app_on_demand(game_instance);
        }
    }

    StdoutWriter::app_exited()?;

    Ok(())
}
