use crate::out::{GamePadAxis, GameState, ProtocolOut, SaveStateInfo, WindowState};
use std::io;
use std::io::Write;
use tinic_generics::error_handle::{ErrorHandle, TinicResult};

pub fn emit_protocol_event(event: &ProtocolOut) -> TinicResult<()> {
    let json = serde_json::to_string(event)
        .map_err(|e| ErrorHandle::new(&format!("Erro ao tentar serializar o evento: {e}")))?;

    let mut stdout = io::stdout();

    writeln!(stdout, "{json}").map_err(|e| {
        ErrorHandle::new(&format!("Erro ao tentar escrever no stdout: [{json}]: {e}"))
    })?;

    stdout
        .flush()
        .map_err(|e| ErrorHandle::new(&format!("Erro ao tentar escrever no stdout: [{json}]: {e}")))
}

pub struct StdoutWriter;

impl StdoutWriter {
    pub fn window_state_change(state: WindowState) -> TinicResult<()> {
        emit_protocol_event(&ProtocolOut::WindowStateChange { state })
    }

    pub fn game_state_change(state: GameState) -> TinicResult<()> {
        emit_protocol_event(&ProtocolOut::GameStateChange { state })
    }

    pub fn save_state_result(info: SaveStateInfo) -> TinicResult<()> {
        emit_protocol_event(&ProtocolOut::SaveStateResult { info })
    }

    pub fn load_state_result(success: bool) -> TinicResult<()> {
        emit_protocol_event(&ProtocolOut::LoadStateResult { success })
    }

    pub fn keyboard_state(using: bool) -> TinicResult<()> {
        emit_protocol_event(&ProtocolOut::KeyboardState { using })
    }

    pub fn device_connected(id: String, name: String) -> TinicResult<()> {
        emit_protocol_event(&ProtocolOut::DeviceConnected { name, id })
    }

    pub fn device_disconnected(id: String, name: String) -> TinicResult<()> {
        emit_protocol_event(&ProtocolOut::DeviceDisconnected { name, id })
    }

    pub fn device_button_pressed(id: String, name: String, button: String) -> TinicResult<()> {
        emit_protocol_event(&ProtocolOut::DeviceButtonPressed { id, name, button })
    }

    pub fn app_exited() -> TinicResult<()> {
        emit_protocol_event(&ProtocolOut::AppExited)
    }

    pub fn device_axis_change(id: String, name: String, axis: GamePadAxis) -> TinicResult<()> {
        emit_protocol_event(&ProtocolOut::DeviceAxisChange { id, name, axis })
    }
}
