use crate::devices_manager::{DeviceRubble, DeviceStateListener, DevicesManager};
use crate::gamepad::retro_gamepad::RetroGamePad;
use crate::state_thread::EventThread;
use retro_ab::erro_handle::ErroHandle;
use retro_ab::retro_sys::retro_rumble_effect;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref DEVICES_MANAGER: Arc<Mutex<DevicesManager>> =
        Arc::new(Mutex::new(DevicesManager::new(None)));
}

#[derive(Debug)]
pub struct RetroAbController {
    event_thread: EventThread,
}

impl Drop for RetroAbController {
    fn drop(&mut self) {
        self.event_thread.stop();
    }
}

impl RetroAbController {
    pub fn new(listener: Option<DeviceStateListener>) -> Result<RetroAbController, ErroHandle> {
        if let Some(listener) = listener {
            DEVICES_MANAGER
                .lock()
                .unwrap()
                .set_listener(Arc::new(Mutex::new(listener)));
        }

        let mut event_thread = EventThread::new();
        event_thread.resume(DEVICES_MANAGER.clone())?;

        Ok(Self { event_thread })
    }

    #[doc = "retorna uma lista de gamepad disponíveis"]
    pub fn get_list(&self) -> Vec<RetroGamePad> {
        DEVICES_MANAGER.lock().unwrap().get_gamepads()
    }

    pub fn set_max_port(max: usize) {
        DEVICES_MANAGER.lock().unwrap().set_max_port(max);
    }

    #[doc = "Para que o CORE possa 'tomar posse' com existo dos eventos do gamepad é necessário interromper o a thread de eventos"]
    pub fn stop_thread_events(&mut self) {
        self.event_thread.stop();
    }

    #[doc = "Devolve a 'posse' dos eventos do gamepad dada ao CORE para a thread de eventos. chame isso quando nao houve nenhuma rom em execução"]
    pub fn resume_thread_events(&mut self) -> Result<(), ErroHandle> {
        self.event_thread.resume(DEVICES_MANAGER.clone())
    }

    pub fn apply_rumble(&self, rubble: DeviceRubble) {
        DEVICES_MANAGER.lock().unwrap().apply_rumble(rubble);
    }
}

//***********ENVIE ESSAS CALLBACKS PARA CORE****************/
pub fn input_poll_callback() {
    DEVICES_MANAGER.lock().unwrap().update_state();
}

pub fn input_state_callback(port: i16, _device: i16, _index: i16, id: i16) -> i16 {
    DEVICES_MANAGER.lock().unwrap().get_input_state(port, id)
}

pub fn rumble_callback(
    port: std::os::raw::c_uint,
    effect: retro_rumble_effect,
    strength: u16,
) -> bool {
    DEVICES_MANAGER.lock().unwrap().apply_rumble(DeviceRubble {
        port: port as usize,
        effect,
        strength,
    })
}
//****************************************************/
