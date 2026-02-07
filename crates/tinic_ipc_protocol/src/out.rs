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
