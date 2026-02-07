use crate::io::stdout_writer::StdoutWriter;
use tinic::{DeviceListener, RetroGamePad};

pub struct DeviceEventHandle;

impl DeviceListener for DeviceEventHandle {
    fn connected(&self, device: RetroGamePad) {
        let _ = StdoutWriter::device_connected(device.id.to_string(), device.name);
    }

    fn disconnected(&self, device: RetroGamePad) {
        let _ = StdoutWriter::device_disconnected(device.id.to_string(), device.name);
    }

    fn button_pressed(&self, button: String, device: RetroGamePad) {
        let _ = StdoutWriter::device_button_pressed(device.id.to_string(), device.name, button);
    }
}
