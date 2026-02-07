use crate::thread_stack::model_stack::{ModelStackManager, RetroStackFn};
use retro_ab::option_manager::Options;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum MainStackCommand {
    GetCoreOption(Arc<Options>),
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

    fn read(&self) -> Vec<MainStackCommand> {
        self.manager.read()
    }

    fn clear(&self) {
        self.manager.clear();
    }
}
