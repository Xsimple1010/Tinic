use crate::thread_stack::model_stack::{ModelStackManager, RetroStackFn};
use retro_ab::paths::Paths;
use retro_ab_gamepad::devices_manager::Device;

//facilita o reconhecimento dos atributos usando a intellisense da ide
type CorePath = String;
type RomPath = String;
type Slot = usize;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameStackCommand {
    //core, rom, paths
    LoadGame(CorePath, RomPath, Paths),
    SaveState(Slot),
    LoadState(Slot),
    Pause,
    Resume,
    DeviceConnected(Device),
    Quit,
    Reset,
}

#[derive(Clone, Debug)]
pub struct GameStack {
    manager: ModelStackManager<GameStackCommand>,
}

impl GameStack {
    pub fn new() -> Self {
        Self {
            manager: ModelStackManager::new(),
        }
    }
}

impl RetroStackFn<GameStackCommand> for GameStack {
    fn push(&self, command: GameStackCommand) {
        self.manager.push(command);
    }

    fn read(&self) -> Vec<GameStackCommand> {
        self.manager.read()
    }

    fn clear(&self) {
        self.manager.clear();
    }
}

#[cfg(test)]
mod retro_stack_test {
    use super::{GameStackCommand, ModelStackManager};
    use std::thread;

    #[test]
    fn clear() {
        let stack = ModelStackManager::new();

        stack.push(GameStackCommand::Quit);

        let stack_2 = stack.clone();

        let _ = thread::spawn(move || {
            let commands = stack_2.read();
            assert_eq!(commands.len(), 1);

            let cmd = commands.first().unwrap().clone();
            assert_eq!(cmd, GameStackCommand::Quit)
        })
        .join();

        assert_eq!(stack.read().is_empty(), true);
    }

    #[test]
    fn push_and_read() {
        let stack = ModelStackManager::new();

        stack.push(GameStackCommand::Quit);

        let commands = stack.read();
        assert_eq!(commands.len(), 1);
        assert_eq!(commands.first().unwrap().clone(), GameStackCommand::Quit);
    }
}
