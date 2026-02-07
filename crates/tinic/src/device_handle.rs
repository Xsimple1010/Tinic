use retro_controllers::devices_manager::{Device, DeviceListener};

#[derive(Debug)]
pub struct TinicDeviceHandle {
    extern_listener: Box<dyn DeviceListener>,
}

impl TinicDeviceHandle {
    pub fn new(extern_listener: Box<dyn DeviceListener>) -> Self {
        Self { extern_listener }
    }
}

impl DeviceListener for TinicDeviceHandle {
    fn connected(&self, device: Device) {
        self.extern_listener.connected(device);
    }

    fn disconnected(&self, device: Device) {
        self.extern_listener.disconnected(device);
    }

    fn button_pressed(&self, button: String, device: Device) {
        self.extern_listener.button_pressed(button, device);
    }
}
