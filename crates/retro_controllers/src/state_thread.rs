use crate::devices_manager::DevicesManager;
use generics::types::TMutex;
use generics::{constants::THREAD_SLEEP_TIME, types::ArcTMutex};
use std::sync::Arc;
use std::{
    thread::{self, sleep},
    time::Duration,
};

#[derive(Debug)]
pub struct EventThread {
    event_thread_can_run: ArcTMutex<bool>,
}

impl EventThread {
    pub fn new() -> Self {
        EventThread {
            event_thread_can_run: TMutex::new(false),
        }
    }

    pub fn stop(&self) {
        self.event_thread_can_run.store(false);
    }

    pub fn resume(&self, devices: Arc<DevicesManager>) {
        self.event_thread_can_run.store(true);
        self.create_update_devices_state_thread(devices);
    }

    /// # event listener thread
    ///
    /// Isso é util se quando não há uma *rom* em execução, mas ainda é necessário ouvir os eventos de
    /// input. Por exemplo, a *rom* foi fechada, mas a interface do usuário ainda precisa ser
    /// notificada sobre os eventos de input.
    ///
    /// Aviso: para evitar uso desnecessário de CPU use isso somente quando não hover uma
    /// *rom* em execução!
    fn create_update_devices_state_thread(&self, devices: Arc<DevicesManager>) {
        let event_thread_is_enabled = self.event_thread_can_run.clone();

        thread::spawn(move || {
            while *event_thread_is_enabled.load_or(false) {
                //WITHOUT THIS, WI HAVE A HIGH CPU UTILIZATION!
                sleep(Duration::from_millis(THREAD_SLEEP_TIME));

                devices.update_state().unwrap();
            }
        });
    }
}
