use crate::app_dispatcher::GameInstanceDispatchers;
use retro_controllers::devices_manager::DeviceListener;
use retro_controllers::RetroGamePad;

#[derive(Debug)]
pub struct DeviceHandle {
    pub extern_listener: Box<dyn DeviceListener>,
    pub game_dispatchers: GameInstanceDispatchers,
}

impl DeviceListener for DeviceHandle {
    fn connected(&self, device: RetroGamePad) {
        if self.game_dispatchers.disable_keyboard().is_err() {
            let msg = format!(
                "O {} foi conectado! e não foi possível desconectar o teclado",
                device.name
            );
            println!("{msg}")
        }

        if self
            .game_dispatchers
            .connect_device(device.clone())
            .is_err()
        {
            let msg = format!(
                "O {} foi possível configurar o seu: {}", device.name,
                device.name
            );
            println!("{msg}")
        }

        self.extern_listener.connected(device);
    }

    fn disconnected(&self, device: RetroGamePad) {
        if self.game_dispatchers.enable_keyboard().is_err() {
            let msg = format!(
                "O {} foi desconectado! e não foi possível conectar o teclado novamente",
                device.name
            );
            println!("{msg}")
        }

        self.extern_listener.disconnected(device);
    }

    fn button_pressed(&self, button: String, device: RetroGamePad) {
        self.extern_listener.button_pressed(button, device);
    }
}
