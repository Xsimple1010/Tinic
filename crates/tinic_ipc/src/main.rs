mod app_state;
mod constants;
mod device_listener;
mod game_loop;
mod io;

use crate::app_state::AppState;
use crate::device_listener::DeviceEventHandle;
use crate::game_loop::game_loop;
use crate::io::stdin_reader::StdinReader;
use crate::io::stdout_writer::StdoutWriter;
use tinic::{ErrorHandle, Tinic, WindowListener};

struct WindowEvents;

impl WindowListener for WindowEvents {
    fn window_closed(&self) {
        let _ = StdoutWriter::window_closed();
    }

    fn window_opened(&self) {
        let _ = StdoutWriter::window_opened();
    }

    fn game_loaded_result(&self, suss: bool) {
        let _ = StdoutWriter::game_loaded(suss);
    }

    fn game_closed(&self) {
        let _ = StdoutWriter::game_closed();
    }

    fn game_paused(&self) {
        let _ = StdoutWriter::game_paused();
    }

    fn game_resumed(&self) {
        let _ = StdoutWriter::game_resumed();
    }

    fn save_state_result(&self, suss: bool) {
        let _ = StdoutWriter::save_state_result(suss);
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
    let controle_event = DeviceEventHandle {
        app_state: app_state.clone(),
    };
    tinic.set_controle_listener(Box::new(controle_event))?;

    let window_event = WindowEvents;
    tinic.set_window_listener(Box::new(window_event));

    // App config
    StdinReader::start(app_state.clone());

    game_loop(app_state, tinic)
}
