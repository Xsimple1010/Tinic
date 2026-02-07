use crate::devices_manager::DevicesManager;
use generics::erro_handle::ErroHandle;
use generics::types::TMutex;
use generics::{constants::THREAD_SLEEP_TIME, types::ArcTMuxte};
use std::sync::Arc;
use std::{
    thread::{self, sleep},
    time::Duration,
};

#[derive(Debug)]
pub struct EventThread {
    event_thread_can_run: ArcTMuxte<bool>,
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

    pub fn resume(&self, devices: Arc<DevicesManager>) -> Result<(), ErroHandle> {
        let event_thread_can_run = *self.event_thread_can_run.load_or(false);

        if event_thread_can_run {
            return Ok(());
        }

        if self.try_enable_thread() && !self.try_enable_thread() {
            return Err(ErroHandle {
                message: "Não foi possível iniciar a thread de eventos do gamepad".to_string(),
            });
        }

        self.create_update_devices_state_thread(devices, self.event_thread_can_run.clone());

        Ok(())
    }

    fn try_enable_thread(&self) -> bool {
        let mut need_try_again = false;

        {
            self.event_thread_can_run.store_or_else(true, |poison| {
                let mut _is_enable = *poison.into_inner();

                if _is_enable {
                    _is_enable = false;
                    need_try_again = true;
                } else {
                    _is_enable = true;
                }
            });
        }

        if need_try_again {
            // A thread gamepad_listener precisará de tempo para ler o mutex novamente.
            sleep(Duration::from_millis(THREAD_SLEEP_TIME));
        }

        need_try_again
    }

    /// # event listener thread
    ///
    /// Isso é util se quando não há uma *rom* em execução, mas ainda é necessário ouvir os eventos de
    /// input. Por exemplo, a *rom* foi fechada, mas a interface do usuário ainda precisa ser
    /// notificada sobre os eventos de input.
    ///
    /// Aviso: para evitar uso desnecessário de CPU use isso somente quando não hover uma
    /// *rom* em execução! Use o terceiro parâmetro 'event-thread-is-enabled' para encerar a
    /// execução da thread quando não precisar mais dela.
    fn create_update_devices_state_thread(
        &self,
        devices: Arc<DevicesManager>,
        event_thread_is_enabled: ArcTMuxte<bool>,
    ) {
        thread::spawn(move || {
            while *event_thread_is_enabled.load_or(false) {
                //WITHOUT THIS, WI HAVE A HIGH CPU UTILIZATION!
                sleep(Duration::from_millis(THREAD_SLEEP_TIME));

                devices.update_state().unwrap();
            }
        });
    }
}
