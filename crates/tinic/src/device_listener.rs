use crate::app_dispatcher::GameInstanceDispatchers;
use generics::types::ArcTMutex;
use retro_controllers::devices_manager::DeviceListener;
use retro_controllers::RetroGamePad;

#[derive(Debug)]
pub struct DeviceHandle {
    pub extern_listener: Box<dyn DeviceListener>,
    pub app_proxy: ArcTMutex<Option<GameInstanceDispatchers>>,
}

impl DeviceListener for DeviceHandle {
    fn connected(&self, device: RetroGamePad) {
        let mut invalid_proxy = false;

        if let Some(dispatcher) = self.app_proxy.load_or(None).as_ref() {
            if dispatcher.disable_keyboard().is_err() {
                invalid_proxy = true;
            }

            if dispatcher.connect_device(device.clone()).is_err() {
                invalid_proxy = true;
            }
        }

        if invalid_proxy {
            self.app_proxy.store(None);
        }

        self.extern_listener.connected(device);
    }

    fn disconnected(&self, device: RetroGamePad) {
        let mut invalid_proxy = false;

        if let Some(dispatcher) = self.app_proxy.load_or(None).as_ref()
            && dispatcher.enable_keyboard().is_err()
        {
            invalid_proxy = true;
        }

        if invalid_proxy {
            self.app_proxy.store(None);
        }

        self.extern_listener.disconnected(device);
    }

    fn button_pressed(&self, button: String, device: RetroGamePad) {
        self.extern_listener.button_pressed(button, device);
    }
}
