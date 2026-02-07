use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone, Debug)]
pub struct ModelStackManager<T> {
    commands: Arc<Mutex<Vec<T>>>,
}

pub trait RetroStackFn<T> {
    fn push(&self, command: T);

    fn read_and_clear(&self) -> Vec<T>;

    fn read(&self) -> Vec<T>;

    fn remove_index(&self, index: usize);

    fn clear(&self);
}

impl<T: Clone> RetroStackFn<T> for ModelStackManager<T> {
    fn push(&self, command: T) {
        self.get_commands_mutex().push(command);
    }

    fn read_and_clear(&self) -> Vec<T> {
        let mut commands = self.get_commands_mutex();
        let v_commands = commands.clone().to_vec();

        commands.clear();

        v_commands
    }

    fn read(&self) -> Vec<T> {
        let commands = self.get_commands_mutex();
        commands.clone().to_vec()
    }

    fn remove_index(&self, index: usize) {
        self.get_commands_mutex().remove(index);
    }

    fn clear(&self) {
        self.get_commands_mutex().clear();
    }
}

impl<T: Clone> ModelStackManager<T> {
    pub fn new() -> ModelStackManager<T> {
        Self {
            commands: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn get_commands_mutex(&self) -> MutexGuard<'_, Vec<T>> {
        self.commands.lock().unwrap_or_else(|op| {
            let mut commands = op.into_inner();
            *commands = Vec::new();
            commands
        })
    }
}

#[cfg(test)]
mod retro_stack_test {
    use super::ModelStackManager;
    use crate::thread_stack::{game_stack::GameStackCommand, model_stack::RetroStackFn};
    use std::thread;

    #[test]
    fn clear() {
        let stack = ModelStackManager::new();

        stack.push(GameStackCommand::Quit);

        let stack_2 = stack.clone();

        let _ = thread::spawn(move || {
            let commands = stack_2.read_and_clear();
            assert_eq!(commands.len(), 1);

            let cmd = commands.first().unwrap().clone();
            assert_eq!(cmd, GameStackCommand::Quit)
        })
        .join();

        assert!(stack.read().is_empty());
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
