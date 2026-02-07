use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone, Debug)]
pub struct ModelStackManager<T> {
    commands: Arc<Mutex<Vec<T>>>,
}

pub trait RetroStackFn<T> {
    fn push(&self, command: T);

    fn read(&self) -> Vec<T>;
    fn clear(&self);
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

    pub fn push(&self, command: T) {
        self.get_commands_mutex().push(command);
    }

    pub fn read(&self) -> Vec<T> {
        let mut commands = self.get_commands_mutex();
        let v_commands = commands.clone().to_vec();

        commands.clear();

        v_commands
    }

    pub fn clear(&self) {
        self.get_commands_mutex().clear();
    }
}
