use tinic::{
    self, args_manager::RetroArgs, DeviceListener, ErrorHandle, RetroGamePad, Tinic, TinicGameInfo,
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

fn main() -> Result<(), ErrorHandle> {
    let args = RetroArgs::new()?;

    let event = DeviceEventHandle::default();
    let mut tinic = Tinic::new(Box::new(event))?;

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
