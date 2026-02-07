use generics::test_workdir::{
    create_test_work_dir_path, get_core_test_path, get_rom_test_path, remove_test_work_dir_path,
};
use tinic::{
    self, DeviceListener, ErrorHandle, GameState, RetroGamePad, SaveStateInfo, Tinic,
    TinicGameInfo, WindowListener, WindowState,
};

#[derive(Debug, Default)]
struct DeviceEventHandle;

impl DeviceListener for DeviceEventHandle {
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

struct WindowEvents;

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

fn main() -> Result<(), ErrorHandle> {
    let event = DeviceEventHandle::default();
    let mut tinic = Tinic::new()?;
    tinic.set_controle_listener(Box::new(event))?;
    tinic.set_window_listener(Box::new(WindowEvents));

    let test_dir = "tinic_example";

    let game_info = TinicGameInfo {
        core: get_core_test_path().display().to_string(),
        rom: get_rom_test_path().display().to_string(),
        sys_dir: create_test_work_dir_path(test_dir).display().to_string(),
    };

    let game_instance = tinic.create_game_instance(game_info)?;
    tinic.run(game_instance)?;

    remove_test_work_dir_path(test_dir)
}
