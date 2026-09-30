use tinic::{DeviceListener, RetroGamePad};
use tinic_ipc_protocol::{helpers::stdout_writer::StdoutWriter, out::GamePadAxis};

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

    fn axis_change(&self, axis: GamePadAxis, device: RetroGamePad) {
        let _ = StdoutWriter::device_axis_change(device.id.to_string(), device.name, axis);
    }
}
