use super::game_thread_state::ThreadState;
use crate::thread_stack::game_stack::GameStackCommand::{
    DeviceConnected, DisableFullScreen, EnableFullScreen, LoadGame, LoadState, Pause, Reset,
    Resume, SaveState,
};
use generics::erro_handle::ErroHandle;

pub fn stack_commands_handle(state: &mut ThreadState) -> Result<(), ErroHandle> {
    for cmd in state.channel_notify.read_game_stack() {
        match cmd {
            LoadGame(core_path, rom_path, paths) => state.load_game(core_path, rom_path, paths)?,
            SaveState(slot) => state.save_state(slot)?,
            LoadState(slot) => state.load_state(slot)?,
            Pause => state.pause()?,
            Resume => state.resume()?,
            Reset => state.reset()?,
            EnableFullScreen => state.enable_full_screen()?,
            DisableFullScreen => state.disable_full_screen()?,
            DeviceConnected(device) => state.connect_device(device)?,
        }
    }

    Ok(())
}
