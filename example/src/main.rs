use std::env;

use tinic::{self, args_manager, test_tools};

fn main() {
    let args = args_manager::get_values(env::args().collect());

    let (_, core_path) = args
        .get_key_value("core")
        .expect("O caminho para o core nao foi fornecido tente --core=caminho_pra_core_aqui!");

    let (_, rom_path) = args
        .get_key_value("rom")
        .expect("O caminho para o rom nao foi fornecido tente --rom=caminho_pra_rom_aqui!");
    println!("Hello, world!");

    tinic::load(
        core_path,
        rom_path.to_string(),
        test_tools::paths::get_paths(),
    );
}
