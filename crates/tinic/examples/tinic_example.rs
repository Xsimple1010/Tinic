use tinic::{
    self, args_manager::RetroArgs, DeviceListener, ErrorHandle, GameState, RetroGamePad, SaveImgPreview, SavePath,
    Tinic, TinicGameInfo, WindowListener, WindowState,
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

    fn save_state_result(&self, info: Option<(SavePath, SaveImgPreview)>) {
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
    let args = RetroArgs::new()?;

    let event = DeviceEventHandle::default();
    let mut tinic = Tinic::new()?;
    tinic.set_controle_listener(Box::new(event))?;
    tinic.set_window_listener(Box::new(WindowEvents));

    if let Some(core) = &args.core {
        let game_info = TinicGameInfo {
            core: core.clone(),
            rom: args.rom,
            sys_dir: "/home/aderval/Downloads/RetroArch_cores".to_string(),
        };

        let game_instance = tinic.create_game_instance(game_info)?;
        tinic.run(game_instance)?;
    }
    Ok(())
}
