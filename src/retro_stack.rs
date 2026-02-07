use retro_ab::paths::Paths;
use retro_ab_gamepad::devices_manager::Device;
use std::sync::{Arc, Mutex, MutexGuard};

//facilita o reconhecimento dos atributos usando a intellisense da ide
type CorePath = String;
type RomPath = String;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StackCommand {
    //core, rom, paths
    LoadGame(CorePath, RomPath, Paths),
    SaveState,
    LoadState,
    Pause,
    Resume,
    GamepadConnected(Device),
    Quit,
    Reset,
}

pub struct RetroStack {
    commands: Mutex<Vec<StackCommand>>,
}

impl RetroStack {
    pub fn new() -> Arc<RetroStack> {
        Arc::new(Self {
            commands: Mutex::new(Vec::new()),
        })
    }

    fn get_commands_mutex(&self) -> MutexGuard<'_, Vec<StackCommand>> {
        self.commands.lock().unwrap_or_else(|op| {
            let mut commands = op.into_inner();
            *commands = Vec::new();
            commands
        })
    }

    pub fn push(&self, command: StackCommand) {
        self.get_commands_mutex().push(command);
    }

    pub fn read(&self) -> Vec<StackCommand> {
        let mut commands = self.get_commands_mutex();
        let v_commands = commands.clone().to_vec();

        commands.clear();

        v_commands
    }

    pub fn clear(&self) {
        self.get_commands_mutex().clear();
    }
}

#[cfg(test)]
mod retro_stack_test {
    use super::{RetroStack, StackCommand};
    use std::thread;

    #[test]
    fn clear() {
        let stack = RetroStack::new();

        stack.push(StackCommand::Quit);

        let stack_2 = stack.clone();

        let _ = thread::spawn(move || {
            let commands = stack_2.read();
            assert_eq!(commands.len(), 1);

            let cmd = commands.first().unwrap().clone();
            assert_eq!(cmd, StackCommand::Quit)
        })
        .join();

        assert_eq!(stack.read().is_empty(), true);
    }

    #[test]
    fn push_and_read() {
        let stack = RetroStack::new();

        stack.push(StackCommand::Quit);

        let commands = stack.read();
        assert_eq!(commands.len(), 1);
        assert_eq!(commands.first().unwrap().clone(), StackCommand::Quit);
    }
}
