mod ipc;

use tinic::{Tinic, TinicResult};
use tinic_ipc_protocol::helpers::stdin_reader_trait::StdinReaderTrait;

use crate::ipc::io::stdin_reader::StdinReader;
use crate::ipc::{
    app_state::AppState, device_listener::DeviceEventHandle, game_loop::game_loop,
    window_event_listener::WindowEvents,
};

fn main() -> TinicResult<()> {
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
