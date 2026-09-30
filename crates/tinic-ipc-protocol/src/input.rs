use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "cmd", rename_all = "snake_case")]
pub enum ProtocolInput {
    LoadGame { info: GameInfo },
    GameClose,
    Exit,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GameInfo {
    pub core: String,
    pub rom: String,
    pub sys_dir: String,
}
