use crate::thread_stack::model_stack::{ModelStackManager, RetroStackFn};
use retro_ab::option_manager::OptionManager;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum MainStackCommand {
    GameLoaded(bool, Option<Arc<OptionManager>>),
}

#[derive(Debug, Clone)]
pub struct MainStack {
    manager: ModelStackManager<MainStackCommand>,
}

impl MainStack {
    pub fn new() -> Self {
        Self {
            manager: ModelStackManager::new(),
        }
    }
}

impl RetroStackFn<MainStackCommand> for MainStack {
    fn push(&self, command: MainStackCommand) {
        self.manager.push(command);
    }

    fn read_and_clear(&self) -> Vec<MainStackCommand> {
        self.manager.read_and_clear()
    }

    fn read(&self) -> Vec<MainStackCommand> {
        self.manager.read()
    }

    fn remove_index(&self, index: usize) {
        self.manager.remove_index(index);
    }

    fn clear(&self) {
        self.manager.clear();
    }
}
