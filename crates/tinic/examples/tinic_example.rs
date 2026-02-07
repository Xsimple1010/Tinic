use generics::erro_handle::ErroHandle;
use tinic::{self, args_manager::RetroArgs, DeviceListener, Tinic};

#[derive(Debug, Default)]
struct DeviceEventHandle;

impl DeviceListener for DeviceEventHandle {
    fn connected(&self, device: tinic::Device) {
        println!("connected -> {}", device.name)
    }

    fn disconnected(&self, device: tinic::Device) {
        println!("disconnected -> {}", device.name)
    }

    fn button_pressed(&self, button: String, device: tinic::Device) {
        println!("{} pressed -> {}", device.name, button)
    }
}

#[tokio::main]
async fn main() -> Result<(), ErroHandle> {
    let args = RetroArgs::new()?;

    let event = DeviceEventHandle::default();
    let mut tinic = Tinic::new(Box::new(event))?;

    if let Some(core) = &args.core {
        let d = tinic.make_context(&core, &args.rom)?;
        tinic.run(d)?;
    }
    Ok(())
}
