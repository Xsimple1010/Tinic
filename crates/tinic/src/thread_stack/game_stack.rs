use crate::thread_stack::model_stack::{ModelStackManager, RetroStackFn};
use generics::retro_paths::RetroPaths;
use retro_ab_gamepad::devices_manager::Device;

//facilita o reconhecimento dos atributos usando a intellisense da ide
type CorePath = String;
type RomPath = String;
type Slot = usize;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameStackCommand {
    //core, rom, paths
    LoadGame(CorePath, RomPath, RetroPaths),
    SaveState(Slot),
    LoadState(Slot),
    Pause,
    Resume,
    EnableFullScreen,
    DisableFullScreen,
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

    fn read_and_clear(&self) -> Vec<GameStackCommand> {
        self.manager.read_and_clear()
    }

    fn read(&self) -> Vec<GameStackCommand> {
        self.manager.read()
    }

    fn remove_index(&self, index: usize) {
        self.manager.remove_index(index);
    }

    fn clear(&self) {
        self.manager.clear();
    }
}
