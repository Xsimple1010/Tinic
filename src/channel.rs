use crate::thread_stack::game_stack::{GameStack, GameStackCommand};
use crate::thread_stack::main_stack::MainStack;
use crate::thread_stack::main_stack::MainStackCommand::GameLoaded;
use crate::thread_stack::model_stack::RetroStackFn;
use retro_ab::option_manager::OptionManager;
use retro_ab::paths::Paths;
use retro_ab_gamepad::devices_manager::Device;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct ThreadChannel {
    game_stack: GameStack,
    main_stack: MainStack,
}

const MAX_TIME_TO_AWAIT: u64 = 3;

fn await_time_lapse<F>(mut callback: F)
where
    F: FnMut() -> bool,
{
    let max_time_lapse = Duration::from_secs(MAX_TIME_TO_AWAIT);
    let mut last_time = Instant::now();

    'running: loop {
        let now = Instant::now();
        let time_lapse = now - last_time;

        if time_lapse >= max_time_lapse {
            break 'running;
        } else {
            if callback() {
                break 'running;
            };
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

    pub fn read_game_stack(&self) -> Vec<GameStackCommand> {
        self.game_stack.read_and_clear()
    }

    pub fn clear_game_stack(&self) {
        self.game_stack.clear()
    }

    //####################### ações relacionadas ao carregamento de uma rom #######################
    pub async fn load_game(
        &self,
        core_path: &str,
        rom_path: &str,
        paths: Paths,
    ) -> (bool, Option<Arc<OptionManager>>) {
        self.game_stack.push(GameStackCommand::LoadGame(
            core_path.to_string(),
            rom_path.to_string(),
            paths,
        ));

        let mut core_options: Option<Arc<OptionManager>> = None;
        let mut rom_loaded = false;

        await_time_lapse(|| {
            let commands = self.main_stack.read();
            for index in 0..commands.len() {
                if let Some(command) = commands.get(index) {
                    return match command {
                        GameLoaded(options) => {
                            self.main_stack.remove_index(index);
                            core_options = options.clone();
                            rom_loaded = options.is_none();

                            true
                        }
                    };
                }
            }

            false
        });

        (rom_loaded, core_options)
    }

    pub fn set_game_is_loaded(&self, options: Option<Arc<OptionManager>>) {
        self.main_stack.push(GameLoaded(options))
    }

    // ################### OUTAS AÇÕES MAIS GENÉRICAS DO CORE FICAM AQUI! ###########################
    pub fn save_state(&self, slot: usize) {
        self.game_stack.push(GameStackCommand::SaveState(slot));
    }

    pub fn load_state(&self, slot: usize) {
        self.game_stack.push(GameStackCommand::LoadState(slot));
    }

    pub fn resume_game(&self) {
        self.game_stack.push(GameStackCommand::Resume);
    }

    pub fn pause_game(&self) {
        self.game_stack.push(GameStackCommand::Pause);
    }

    pub fn reset_game(&self) {
        self.game_stack.push(GameStackCommand::Reset);
    }

    pub fn quit(&self) {
        self.game_stack.push(GameStackCommand::Quit);
    }
    pub fn connect_device(&self, device: Device) {
        self.game_stack
            .push(GameStackCommand::DeviceConnected(device))
    }

    //######################### AÇÕES RELACIONAS AO VIDEO FIRAM AQUI! ##############################
    pub fn enable_full_screen(&self) {
        self.game_stack.push(GameStackCommand::EnableFullScreen);
    }

    pub fn disable_full_screen(&self) {
        self.game_stack.push(GameStackCommand::DisableFullScreen);
    }
}
