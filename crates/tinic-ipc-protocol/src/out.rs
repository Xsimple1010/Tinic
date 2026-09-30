use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum ProtocolOut {
    DeviceConnected {
        id: String,
        name: String,
    },
    DeviceDisconnected {
        id: String,
        name: String,
    },
    DeviceButtonPressed {
        id: String,
        name: String,
        button: String,
    },
    DeviceAxisChange {
        id: String,
        name: String,
        axis: GamePadAxis,
    },
    WindowStateChange {
        state: WindowState,
    },
    GameStateChange {
        state: GameState,
    },
    SaveStateResult {
        info: SaveStateInfo,
    },
    LoadStateResult {
        success: bool,
    },
    KeyboardState {
        using: bool,
    },
    // *********
    AppExited,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowState {
    Opened,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum GameState {
    #[default]
    Closed,
    Running,
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

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct GamePadAxis {
    pub left_stick_x: f32,
    pub left_stick_y: f32,
    pub left_z: f32,
    pub right_stick_x: f32,
    pub right_stick_y: f32,
    pub right_z: f32,
    pub dpad_x: f32,
    pub dpad_y: f32,
}
