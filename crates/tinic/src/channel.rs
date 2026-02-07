use crate::thread_stack::game_stack::GameStackCommand::{
    DeviceConnected, DisableFullScreen, EnableFullScreen, LoadGame, Pause, Quit, Reset, Resume,
    SaveState,
};
use crate::thread_stack::game_stack::{GameStack, GameStackCommand};
use crate::thread_stack::main_stack::MainStackCommand::{
    GameLoaded, GameStateSaved, SaveStateLoaded,
};
use crate::thread_stack::main_stack::{MainStack, MainStackCommand};
use crate::thread_stack::model_stack::RetroStackFn;
use generics::constants::MAX_TIME_TO_AWAIT_THREAD_RESPONSE;
use generics::retro_paths::RetroPaths;
use retro_ab::option_manager::OptionManager;
use retro_ab_gamepad::devices_manager::Device;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct ThreadChannel {
    game_stack: GameStack,
    main_stack: MainStack,
}

fn wait_response<C, S: RetroStackFn<C>, CA>(stack: &S, mut callback: CA)
where
    CA: FnMut(&C) -> bool,
{
    let max_time_lapse = Duration::from_secs(MAX_TIME_TO_AWAIT_THREAD_RESPONSE);
    let mut last_time = Instant::now();

    'running: loop {
        let now = Instant::now();
        let time_lapse = now - last_time;

        if time_lapse >= max_time_lapse {
            break 'running;
        } else {
            let commands = stack.read();
            for index in 0..commands.len() {
                if let Some(cmd) = commands.get(index) {
                    if callback(cmd) {
                        stack.remove_index(index);
                        break 'running;
                    };
                }
            }
        }

        last_time = now;
    }
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
    ) -> (bool, Option<Arc<OptionManager>>) {
        self.game_stack
            .push(LoadGame(core_path.to_string(), rom_path.to_string(), paths));

        let mut core_options: Option<Arc<OptionManager>> = None;
        let mut rom_loaded = false;

        wait_response(&self.main_stack, |command| {
            return match command {
                GameLoaded(options) => {
                    core_options = options.clone();
                    rom_loaded = options.is_none();

                    true
                }
                _ => false,
            };
        });

        (rom_loaded, core_options)
    }

    // ################### OUTAS AÇÕES MAIS GENÉRICAS DO CORE FICAM AQUI! ###########################
    pub async fn save_state(&self, slot: usize) -> Option<(String, String)> {
        self.game_stack.push(SaveState(slot));

        let mut save: Option<(String, String)> = None;

        wait_response(&self.main_stack, |command| {
            return match command {
                GameStateSaved(s) => {
                    save = s.to_owned();
                    true
                }
                _ => false,
            };
        });

        save
    }

    pub async fn load_state(&self, slot: usize) -> bool {
        self.game_stack.push(GameStackCommand::LoadState(slot));

        let mut loaded = false;

        wait_response(&self.main_stack, |command| {
            return match command {
                SaveStateLoaded(s_loaded) => {
                    loaded = s_loaded.clone();

                    true
                }
                _ => false,
            };
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

    pub fn quit(&self) {
        self.game_stack.push(Quit);
    }
    pub fn connect_device(&self, device: Device) {
        self.game_stack.push(DeviceConnected(device))
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
