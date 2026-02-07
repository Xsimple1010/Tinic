use serde::Serialize;

#[derive(Debug, Serialize)]
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
    // Window && Game
    WindowOpened,
    WindowClosed,
    GameLoadedResult {
        success: bool,
    },
    GameClosed,
    GamePaused,
    GameResumed,
    SaveStateResult {
        success: bool,
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
