use crate::io::protocol::out::ProtocolOut;
use std::io;
use std::io::Write;
use tinic::ErrorHandle;

pub fn emit_protocol_event(event: &ProtocolOut) -> Result<(), ErrorHandle> {
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
    pub fn window_opened() -> Result<(), ErrorHandle> {
        emit_protocol_event(&ProtocolOut::WindowOpened)
    }

    pub fn window_closed() -> Result<(), ErrorHandle> {
        emit_protocol_event(&ProtocolOut::WindowClosed)
    }

    pub fn game_loaded(success: bool) -> Result<(), ErrorHandle> {
        emit_protocol_event(&ProtocolOut::GameLoadedResult { success })
    }

    pub fn game_closed() -> Result<(), ErrorHandle> {
        emit_protocol_event(&ProtocolOut::GameClosed)
    }

    pub fn game_paused() -> Result<(), ErrorHandle> {
        emit_protocol_event(&ProtocolOut::GamePaused)
    }

    pub fn game_resumed() -> Result<(), ErrorHandle> {
        emit_protocol_event(&ProtocolOut::GameResumed)
    }

    pub fn save_state_result(success: bool) -> Result<(), ErrorHandle> {
        emit_protocol_event(&ProtocolOut::SaveStateResult { success })
    }

    pub fn load_state_result(success: bool) -> Result<(), ErrorHandle> {
        emit_protocol_event(&ProtocolOut::LoadStateResult { success })
    }

    pub fn keyboard_state(using: bool) -> Result<(), ErrorHandle> {
        emit_protocol_event(&ProtocolOut::KeyboardState { using })
    }

    pub fn device_connected(id: String, name: String) -> Result<(), ErrorHandle> {
        emit_protocol_event(&ProtocolOut::DeviceConnected { name, id })
    }

    pub fn device_disconnected(id: String, name: String) -> Result<(), ErrorHandle> {
        emit_protocol_event(&ProtocolOut::DeviceDisconnected { name, id })
    }

    pub fn device_button_pressed(
        id: String,
        name: String,
        button: String,
    ) -> Result<(), ErrorHandle> {
        emit_protocol_event(&ProtocolOut::DeviceButtonPressed { id, name, button })
    }

    pub fn app_exited() -> Result<(), ErrorHandle> {
        emit_protocol_event(&ProtocolOut::AppExited)
    }
}
