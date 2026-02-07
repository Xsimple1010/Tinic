use tinic::{DeviceListener, GameState, RetroGamePad, SaveStateInfo, WindowListener, WindowState};

#[derive(Debug, Default)]
pub struct DeviceEvents;

impl DeviceListener for DeviceEvents {
    fn connected(&self, device: RetroGamePad) {
        println!("connected -> {}", device.name)
    }

    fn disconnected(&self, device: RetroGamePad) {
        println!("disconnected -> {}", device.name)
    }

    fn button_pressed(&self, button: String, device: RetroGamePad) {
        println!("{} pressed -> {}", device.name, button)
    }
}

pub struct WindowEvents;

impl WindowListener for WindowEvents {
    fn window_state_change(&self, state: WindowState) {
        println!("WindowState: {state:?}");
    }

    fn game_state_change(&self, state: GameState) {
        println!("GameState: {state:?}");
    }

    fn save_state_result(&self, info: SaveStateInfo) {
        println!("save_state_result: {info:?}");
    }

    fn load_state_result(&self, suss: bool) {
        println!("load_state_result: {suss}");
    }

    fn keyboard_state(&self, has_using: bool) {
        println!("keyboard_state: has_using -> {has_using}");
    }
}
