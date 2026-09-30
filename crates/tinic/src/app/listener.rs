use tinic_ipc_protocol::out::{GameState, SaveStateInfo, WindowState};

pub trait WindowListener: Send + Sync {
    fn window_state_change(&self, state: WindowState);

    fn game_state_change(&self, state: GameState);

    fn save_state_result(&self, state: SaveStateInfo);

    fn load_state_result(&self, suss: bool);

    fn keyboard_state(&self, has_using: bool);
}
