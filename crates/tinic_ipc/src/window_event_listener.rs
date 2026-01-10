use crate::{app_state::AppStateHandle, io::stdout_writer::StdoutWriter};
use std::sync::atomic::Ordering;
use tinic::{GameState, SaveStateInfo, WindowListener, WindowState};

pub struct WindowEvents {
    pub app_state: AppStateHandle,
}

impl WindowListener for WindowEvents {
    fn window_state_change(&self, state: WindowState) {
        let _ = StdoutWriter::window_state_change(state);
    }

    fn game_state_change(&self, state: GameState) {
        match &state {
            GameState::Closed => {
                self.app_state.game_loaded.store(false, Ordering::SeqCst);
            }
            GameState::Running => {
                self.app_state.game_loaded.store(true, Ordering::SeqCst);
            }
            _ => {}
        };

        let _ = StdoutWriter::game_state_change(state);
    }

    fn save_state_result(&self, info: SaveStateInfo) {
        let _ = StdoutWriter::save_state_result(info);
    }

    fn load_state_result(&self, suss: bool) {
        let _ = StdoutWriter::load_state_result(suss);
    }

    fn keyboard_state(&self, has_using: bool) {
        let _ = StdoutWriter::keyboard_state(has_using);
    }
}
