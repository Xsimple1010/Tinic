use serde::{Deserialize, Serialize};
pub use tinic::{GameState, SaveStateInfo, WindowState};

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
        left_stick_x: f32,
        left_stick_y: f32,
        left_z: f32,
        right_stick_x: f32,
        right_stick_y: f32,
        right_z: f32,
        dpad_x: f32,
        dpad_y: f32,
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
