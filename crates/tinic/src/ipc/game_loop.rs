use crate::ipc::app_state::AppStateHandle;
use std::sync::atomic::Ordering;
use std::thread::sleep;
use std::time::Duration;
use tinic::{Tinic, TinicGameInfo, TinicResult};
use tinic_ipc_protocol::helpers::stdin_reader_trait::THREAD_SLEEP_TIME_IN_MILLISECONDS;
use tinic_ipc_protocol::helpers::stdout_writer::StdoutWriter;

pub fn game_loop(app_state: AppStateHandle, mut tinic: Tinic) -> TinicResult<()> {
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
            let game_instance = tinic.create_game_instance(TinicGameInfo {
                core: game_info.core,
                rom: game_info.rom,
                sys_dir: game_info.sys_dir,
            })?;

            tinic.run(game_instance)?;
            break;
        }
    }

    StdoutWriter::app_exited()?;

    Ok(())
}
