use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy)]
pub enum StackCommand {
    // LoadGame,
    SaveState,
    LoadState,
    Pause,
    Resume,
    // UnloadGame,
    UpdateControllers,
    GameQuit,
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

// pub fn init_game_thread
