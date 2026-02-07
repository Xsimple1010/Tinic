use generics::error_handle::ErrorHandle;
use retro_controllers::RetroGamePad;
use tinic::{
    self, args_manager::RetroArgs, test_tools::paths, DeviceListener, Tinic, TinicPumpStatus,
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
        let mut game_instance = tinic.build(core.clone(), args.rom, paths::get_paths()?)?;
        // tinic.run(game_instance)?;

        loop {
            let status = tinic.pop_event(&mut game_instance);

            if let TinicPumpStatus::Exit(_) = status {
                break;
            }
        }
    }
    Ok(())
}
