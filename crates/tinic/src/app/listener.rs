use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowState {
    Opened,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameState {
    Running,
    Closed,
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SaveStateInfo {
    pub file: String,
    pub img: String,
}

pub trait WindowListener {
    fn window_state_change(&self, state: WindowState);

    fn game_state_change(&self, state: GameState);

    fn save_state_result(&self, info: Option<SaveStateInfo>);

    fn load_state_result(&self, suss: bool);

    fn keyboard_state(&self, has_using: bool);
}
