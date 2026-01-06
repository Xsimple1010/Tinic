use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "cmd", rename_all = "snake_case")]
pub enum ProtocolInput {
    LoadGame {
        rom_path: String,
        core_path: String,
        base_retro_path: String,
    },
    GameClose,
    Exit,
}
