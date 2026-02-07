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

pub type SavePath = String;
pub type SaveImgPreview = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SaveStateInfo {
    Susses {
        save_path: String,
        save_img_preview: String,
    },
    Failed,
}

pub trait WindowListener: Send + Sync {
    fn window_state_change(&self, state: WindowState);

    fn game_state_change(&self, state: GameState);

    fn save_state_result(&self, state: SaveStateInfo);

    fn load_state_result(&self, suss: bool);

    fn keyboard_state(&self, has_using: bool);
}
