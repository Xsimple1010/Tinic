use std::{env, io};
use tinic::{self, args_manager, test_tools, Tinic};

fn main() -> Result<(), String> {
    let args = args_manager::get_values(env::args().collect());

    let (_, core_path) = args
        .get_key_value("core")
        .expect("O caminho para o core nao foi fornecido tente --core=caminho_pra_core_aqui!");

    let (_, rom_path) = args
        .get_key_value("rom")
        .expect("O caminho para o rom nao foi fornecido tente --rom=caminho_pra_rom_aqui!");

    let mut tinic = Tinic::new();

    tinic.load(
        core_path,
        rom_path.to_string(),
        test_tools::paths::get_paths(),
    )?;

    'running: loop {
        println!("Para interagir digite o numero de um dos comandos disponíveis!");
        println!("0: sair");
        println!("1: save state");
        println!("2: load state");
        println!("3: pause");
        println!("4: resume");

        let mut command = String::new();

        match io::stdin().read_line(&mut command) {
            Ok(..) => {
                if command.starts_with("0") {
                    tinic.quit_game();
                    break 'running;
                } else if command.starts_with("1") {
                    tinic.save_state();
                } else if command.starts_with("2") {
                    tinic.load_state();
                } else if command.starts_with("3") {
                    tinic.pause();
                } else if command.starts_with("4") {
                    tinic.resume();
                }
            }
            Err(..) => println!("erro ao ler o comando!"),
        }
    }

    Ok(())
}
