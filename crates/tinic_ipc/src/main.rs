mod app_state;
mod constants;
mod device_listener;
mod game_loop;
mod io;
mod window_event_listener;

use crate::device_listener::DeviceEventHandle;
use crate::game_loop::game_loop;
use crate::io::stdin_reader::StdinReader;
use crate::{app_state::AppState, window_event_listener::WindowEvents};
use tinic::{ErrorHandle, Tinic};

fn main() -> Result<(), ErrorHandle> {
    // tinic config
    let mut tinic = Tinic::new()?;

    // setup controle events
    let game_dispatchers = tinic.get_game_dispatchers();
    let app_state = AppState::new(game_dispatchers);
    tinic.set_controller_listener(Box::new(DeviceEventHandle))?;

    let window_event = WindowEvents {
        app_state: app_state.clone(),
    };
    tinic.set_window_listener(Box::new(window_event));

    // App config
    StdinReader::start(app_state.clone());

    game_loop(app_state, tinic)
}
