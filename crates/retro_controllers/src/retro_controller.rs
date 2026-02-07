use crate::devices_manager::{DeviceListener, DeviceRubble, DevicesManager};
use crate::gamepad::retro_gamepad::RetroGamePad;
use crate::keyboard::Keyboard;
use crate::state_thread::EventThread;
use generics::error_handle::ErrorHandle;
use libretro_sys::binding_libretro::retro_rumble_effect;
use retro_core::RetroControllerEnvCallbacks;
use std::sync::Arc;
use winit::keyboard::PhysicalKey;

pub struct RetroController {
    event_thread: EventThread,
    manager: Arc<DevicesManager>,
}

impl Drop for RetroController {
    fn drop(&mut self) {
        self.event_thread.stop();
    }
}

impl RetroController {
    pub fn new(listener: Box<dyn DeviceListener>) -> Result<RetroController, ErrorHandle> {
        let manager = Arc::new(DevicesManager::new(listener)?);

        let event_thread = EventThread::new();
        event_thread.resume(manager.clone());

        Ok(Self {
            event_thread,
            manager,
        })
    }

    #[doc = "retorna uma lista de gamepad disponíveis"]
    pub fn get_list(&self) -> Result<Vec<RetroGamePad>, ErrorHandle> {
        Ok(self.manager.get_gamepads())
    }

    pub fn set_max_port(&self, max: usize) -> Result<(), ErrorHandle> {
        self.manager.set_max_port(max);
        Ok(())
    }

    #[doc = "Para que o CORE possa 'tomar posse' com existo dos eventos do gamepad é necessário interromper o a thread de eventos"]
    pub fn stop_thread_events(&self) {
        self.event_thread.stop();
    }

    #[doc = "Devolve a 'posse' dos eventos do gamepad dada ao CORE para a thread de eventos. chame isso quando nao houve nenhuma rom em execução"]
    pub fn resume_thread_events(&self) {
        self.event_thread.resume(self.manager.clone())
    }

    pub fn apply_rumble(&self, rubble: DeviceRubble) -> Result<(), ErrorHandle> {
        self.manager.apply_rumble(rubble);
        Ok(())
    }

    pub fn is_using_keyboard(&self) -> bool {
        self.manager.is_using_keyboard()
    }

    pub fn update_keyboard(&self, native: PhysicalKey, pressed: bool) {
        self.manager.update_keyboard(native, pressed)
    }

    pub fn active_keyboard(&self) -> Keyboard {
        self.manager.active_keyboard()
    }

    pub fn disable_keyboard(&self) {
        self.manager.disable_keyboard()
    }

    pub fn get_core_cb(&self) -> RetroControllerCb {
        RetroControllerCb {
            manager: self.manager.clone(),
        }
    }
}
pub struct RetroControllerCb {
    manager: Arc<DevicesManager>,
}

impl RetroControllerEnvCallbacks for RetroControllerCb {
    fn input_poll_callback(&self) -> Result<(), ErrorHandle> {
        self.manager.update_state()?;
        Ok(())
    }

    fn input_state_callback(
        &self,
        port: i16,
        _device: i16,
        _index: i16,
        id: i16,
    ) -> Result<i16, ErrorHandle> {
        Ok(self.manager.get_input_state(port, id))
    }

    fn rumble_callback(
        &self,
        port: std::os::raw::c_uint,
        effect: retro_rumble_effect,
        strength: u16,
    ) -> Result<bool, ErrorHandle> {
        Ok(self.manager.apply_rumble(DeviceRubble {
            port: port as usize,
            effect,
            strength,
        }))
    }
}
