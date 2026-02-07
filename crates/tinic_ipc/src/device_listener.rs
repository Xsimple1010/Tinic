use crate::app_state::AppStateHandle;
use crate::io::stdout_writer::StdoutWriter;
use tinic::{DeviceListener, RetroGamePad};

pub struct DeviceEventHandle {
    pub app_state: AppStateHandle,
}

impl DeviceListener for DeviceEventHandle {
    fn connected(&self, device: RetroGamePad) {
        StdoutWriter::device_connected(device.id.to_string(), device.name);
    }

    fn disconnected(&self, device: RetroGamePad) {
        StdoutWriter::device_disconnected(device.id.to_string(), device.name);
    }

    fn button_pressed(&self, button: String, device: RetroGamePad) {
        StdoutWriter::device_button_pressed(device.id.to_string(), device.name, button);
    }
}
