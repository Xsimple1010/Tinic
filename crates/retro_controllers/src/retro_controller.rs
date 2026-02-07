use crate::devices_manager::{DeviceRubble, DeviceStateListener, DevicesManager};
use crate::gamepad::retro_gamepad::RetroGamePad;
use crate::state_thread::EventThread;
use generics::erro_handle::ErroHandle;
use generics::types::{ArcTMuxte, TMutex};
use libretro_sys::binding_libretro::retro_rumble_effect;

lazy_static! {
    static ref DEVICES_MANAGER: ArcTMuxte<DevicesManager> =
        TMutex::new(DevicesManager::new().unwrap());
}

#[derive(Debug)]
pub struct RetroController {
    event_thread: EventThread,
}

impl Drop for RetroController {
    fn drop(&mut self) {
        self.event_thread.stop();
    }
}

impl RetroController {
    pub fn new(listener: DeviceStateListener) -> Result<RetroController, ErroHandle> {
        DEVICES_MANAGER.try_load()?.set_listener(listener);

        let mut event_thread = EventThread::new();
        event_thread.resume(DEVICES_MANAGER.clone())?;

        Ok(Self { event_thread })
    }

    #[doc = "retorna uma lista de gamepad disponíveis"]
    pub fn get_list(&self) -> Result<Vec<RetroGamePad>, ErroHandle> {
        Ok(DEVICES_MANAGER.try_load()?.get_gamepads())
    }

    pub fn set_max_port(max: usize) -> Result<(), ErroHandle> {
        DEVICES_MANAGER.try_load()?.set_max_port(max);
        Ok(())
    }

    #[doc = "Para que o CORE possa 'tomar posse' com existo dos eventos do gamepad é necessário interromper o a thread de eventos"]
    pub fn stop_thread_events(&mut self) {
        self.event_thread.stop();
    }

    #[doc = "Devolve a 'posse' dos eventos do gamepad dada ao CORE para a thread de eventos. chame isso quando nao houve nenhuma rom em execução"]
    pub fn resume_thread_events(&mut self) -> Result<(), ErroHandle> {
        self.event_thread.resume(DEVICES_MANAGER.clone())
    }

    pub fn apply_rumble(&self, rubble: DeviceRubble) -> Result<(), ErroHandle> {
        DEVICES_MANAGER.try_load()?.apply_rumble(rubble);
        Ok(())
    }
}

//***********ENVIE ESSAS CALLBACKS PARA CORE****************/
pub fn input_poll_callback() {
    if let Ok(mut manager) = DEVICES_MANAGER.try_load() {
        let _ = manager.update_state();
    }
}

pub fn input_state_callback(port: i16, _device: i16, _index: i16, id: i16) -> i16 {
    if let Ok(manager) = DEVICES_MANAGER.try_load() {
        manager.get_input_state(port, id)
    } else {
        0
    }
}

pub fn rumble_callback(
    port: std::os::raw::c_uint,
    effect: retro_rumble_effect,
    strength: u16,
) -> bool {
    if let Ok(manager) = DEVICES_MANAGER.try_load() {
        manager.apply_rumble(DeviceRubble {
            port: port as usize,
            effect,
            strength,
        })
    } else {
        false
    }
}
//****************************************************/
