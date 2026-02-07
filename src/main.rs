extern crate retro_ab;
extern crate retro_ab_av;
extern crate retro_ab_gamepad;

mod game_loop;
mod thread;

use game_loop::init_game_loop;
use retro_ab::{
    args_manager,
    core::{self, RetroEnvCallbacks},
    erro_handle::ErroHandle,
    test_tools::paths,
};
use retro_ab_av::{audio_sample_batch_callback, audio_sample_callback, video_refresh_callback};
use retro_ab_gamepad::context::{input_poll_callback, input_state_callback, GamepadContext};
use std::env;

fn main() -> Result<(), ErroHandle> {
    let args = args_manager::get_values(env::args().collect());

    let (_, core_path) = args
        .get_key_value("core")
        .expect("O caminho para o core nao foi fornecido tente --core=caminho_pra_core_aqui!");

    let (_, rom_path) = args
        .get_key_value("rom")
        .expect("O caminho para o rom nao foi fornecido tente --rom=caminho_pra_rom_aqui!");

    let core_ctx = core::load(
        core_path,
        paths::get_paths(),
        RetroEnvCallbacks {
            audio_sample_batch_callback,
            audio_sample_callback,
            input_poll_callback,
            input_state_callback,
            video_refresh_callback,
        },
    )
    .expect("Erro ao tentar inicia o Core");
    core::init(&core_ctx).expect("msg");
    core::load_game(&core_ctx, rom_path).expect("msg");

    let mut controller_ctx = GamepadContext::new(core_ctx.core.system.ports.lock().unwrap().len());

    init_game_loop(core_ctx, controller_ctx.search());

    Ok(())
}
