mod app_state;
mod constants;
mod device_listener;
mod game_loop;
mod io;

use crate::app_state::{AppState, AppStateHandle};
use crate::device_listener::DeviceEventHandle;
use crate::game_loop::game_loop;
use crate::io::stdin_reader::StdinReader;
use crate::io::stdout_writer::StdoutWriter;
use tinic::{ErrorHandle, GameState, Tinic, WindowListener, WindowState};

struct WindowEvents {
    app_state: AppStateHandle,
}

impl WindowListener for WindowEvents {
    fn window_state_change(&self, state: WindowState) {
        let _ = StdoutWriter::window_state_change(state);
    }

    fn game_state_change(&self, state: GameState) {
        let _ = StdoutWriter::game_state_change(state);
    }

    fn save_state_result(&self, info: Option<tinic::SaveStateInfo>) {
        let _ = StdoutWriter::save_state_result(info);
    }

    fn load_state_result(&self, suss: bool) {
        let _ = StdoutWriter::load_state_result(suss);
    }

    fn keyboard_state(&self, has_using: bool) {
        let _ = StdoutWriter::keyboard_state(has_using);
    }
}

fn main() -> Result<(), ErrorHandle> {
    // tinic config
    let mut tinic = Tinic::new()?;

    // setup controle events
    let game_dispatchers = tinic.get_game_dispatchers();
    let app_state = AppState::new(game_dispatchers);
    tinic.set_controle_listener(Box::new(DeviceEventHandle))?;

    let window_event = WindowEvents {
        app_state: app_state.clone(),
    };
    tinic.set_window_listener(Box::new(window_event));

    // App config
    StdinReader::start(app_state.clone());

    game_loop(app_state, tinic)
}
