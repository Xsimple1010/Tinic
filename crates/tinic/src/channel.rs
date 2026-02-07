use crate::thread_stack::game_stack::GameStackCommand::{
    DeviceConnected, DisableFullScreen, EnableFullScreen, LoadGame, Pause, Reset, Resume, SaveState,
};
use crate::thread_stack::game_stack::{GameStack, GameStackCommand};
use crate::thread_stack::main_stack::MainStackCommand::{
    GameLoaded, GameStateSaved, QuitSusses, SaveStateLoaded,
};
use crate::thread_stack::main_stack::{MainStack, MainStackCommand, SaveImg, SavePath};
use crate::thread_stack::model_stack::{wait_response, RetroStackFn};
use generics::retro_paths::RetroPaths;
use retro_controllers::devices_manager::Device;
use retro_core::option_manager::OptionManager;
use std::sync::Arc;

#[derive(Debug)]
pub struct ThreadChannel {
    game_stack: GameStack,
    main_stack: MainStack,
}

impl ThreadChannel {
    pub fn new() -> Self {
        Self {
            main_stack: MainStack::new(),
            game_stack: GameStack::new(),
        }
    }

    pub fn get_notify(&self) -> ChannelNotify {
        ChannelNotify::from(self.game_stack.clone(), self.main_stack.clone())
    }

    //####################### ações relacionadas ao carregamento de uma rom #######################
    pub async fn load_game(
        &self,
        core_path: &str,
        rom_path: &str,
        paths: RetroPaths,
    ) -> Option<Arc<OptionManager>> {
        self.game_stack
            .push(LoadGame(core_path.to_string(), rom_path.to_string(), paths));

        let mut core_options: Option<Arc<OptionManager>> = None;

        wait_response(&self.main_stack, |command| match command {
            GameLoaded(options) => {
                core_options = options.clone();

                true
            }
            _ => false,
        });

        core_options
    }

    // ################### OUTAS AÇÕES MAIS GENÉRICAS DO CORE FICAM AQUI! ###########################
    pub async fn save_state(&self, slot: usize) -> Option<(SavePath, SaveImg)> {
        self.game_stack.push(SaveState(slot));

        let mut save: Option<(SavePath, SaveImg)> = None;

        wait_response(&self.main_stack, |command| match command {
            GameStateSaved(s) => {
                save = s.to_owned();
                true
            }
            _ => false,
        });

        save
    }

    pub async fn load_state(&self, slot: usize) -> bool {
        self.game_stack.push(GameStackCommand::LoadState(slot));

        let mut loaded = false;

        wait_response(&self.main_stack, |command| match command {
            SaveStateLoaded(s_loaded) => {
                loaded = *s_loaded;

                true
            }
            _ => false,
        });

        loaded
    }

    pub fn resume_game(&self) {
        self.game_stack.push(Resume);
    }

    pub fn pause_game(&self) {
        self.game_stack.push(Pause);
    }

    pub fn reset_game(&self) {
        self.game_stack.push(Reset);
    }

    pub fn connect_device(&self, device: Device) {
        self.game_stack.push(DeviceConnected(device))
    }

    pub async fn quit(&self) -> bool {
        self.game_stack.push(GameStackCommand::Quit);

        let mut suss = false;

        wait_response(&self.main_stack, |command| match command {
            QuitSusses(s) => {
                suss = *s;
                true
            }
            _ => false,
        });

        self.main_stack.clear();

        suss
    }

    //######################### AÇÕES RELACIONAS AO VIDEO FICAM AQUI! ##############################
    pub fn enable_full_screen(&self) {
        self.game_stack.push(EnableFullScreen);
    }

    pub fn disable_full_screen(&self) {
        self.game_stack.push(DisableFullScreen);
    }
}

pub struct ChannelNotify {
    game_stack: GameStack,
    main_stack: MainStack,
}

impl ChannelNotify {
    fn from(game_stack: GameStack, main_stack: MainStack) -> Self {
        Self {
            game_stack,
            main_stack,
        }
    }

    pub fn notify_main_stack(&self, command: MainStackCommand) {
        self.main_stack.push(command);
    }

    pub fn notify_game_stack(&self, command: GameStackCommand) {
        self.game_stack.push(command);
    }

    pub fn read_game_stack(&self) -> Vec<GameStackCommand> {
        self.game_stack.read_and_clear()
    }

    pub fn clear_game_stack(&self) {
        self.game_stack.clear()
    }
}
