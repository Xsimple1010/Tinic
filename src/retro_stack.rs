use std::sync::{Arc, Mutex};

use retro_ab::{core::RetroEnvCallbacks, paths::Paths};
use retro_ab_gamepad::retro_gamepad::RetroGamePad;

//facilita o reconhecimento dos atributos usando a intellisense da ide
type CorePath = String;
type RomPath = String;

#[derive(Clone)]
pub enum StackCommand {
    //core, rom, paths, callbacks
    LoadGame(CorePath, RomPath, Paths, RetroEnvCallbacks),
    SaveState,
    LoadState,
    Pause,
    Resume,
    GamepadConnected(RetroGamePad),
    Quit,
    Reset,
}

pub struct RetroStack {
    command: Mutex<Vec<StackCommand>>,
}

impl RetroStack {
    pub fn new() -> Arc<RetroStack> {
        Arc::new(Self {
            command: Mutex::new(Vec::new()),
        })
    }

    pub fn push(&self, command: StackCommand) {
        self.command.lock().unwrap().push(command);
    }

    pub fn read(&self) -> Vec<StackCommand> {
        let m_cmd = &mut *self.command.lock().unwrap();
        let cmd = m_cmd.clone().to_vec();

        m_cmd.clear();

        cmd
    }
}
