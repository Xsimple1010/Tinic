use retro_ab::erro_handle::ErroHandle;
use std::io;
use tinic::{
    self, args_manager::RetroArgs, test_tools::paths::get_paths, Device, DeviceState, Tinic,
};

fn gamepad_state_listener(state: DeviceState, device: Device) {
    println!("{:?} - {:?}", device.name, state);
}

fn main() -> Result<(), ErroHandle> {
    let args = RetroArgs::new()?;

    let mut tinic = Tinic::new(Some(gamepad_state_listener))?;

    tinic.load_core(args.core, args.rom, get_paths()?)?;

    'running: loop {
        println!("Para interagir digite o numero de um dos comandos disponíveis!");
        println!("0: sair");
        println!("1: save state");
        println!("2: load state");
        println!("3: pause");
        println!("4: resume");
        println!("5: reset");
        println!("6: stop rom");

        let mut command = String::new();

        match io::stdin().read_line(&mut command) {
            Ok(..) => {
                if command.starts_with("0") {
                    tinic.quit();
                    break 'running;
                } else if command.starts_with("1") {
                    tinic.save_state();
                } else if command.starts_with("2") {
                    tinic.load_state();
                } else if command.starts_with("3") {
                    tinic.pause();
                } else if command.starts_with("4") {
                    tinic.resume();
                } else if command.starts_with("5") {
                    tinic.reset();
                } else if command.starts_with("6") {
                    tinic.quit();
                }

                println!();
            }
            Err(..) => println!("erro ao ler o comando!"),
        }
    }

    Ok(())
}
