use crate::{
    channel::ThreadChannel,
    game_thread::game_thread_handle::GameThread,
    generics::{erro_handle::ErroHandle, retro_paths::RetroPaths},
    libretro_sys::binding_libretro::retro_log_level::RETRO_LOG_ERROR,
    retro_controllers::{
        devices_manager::{Device, DeviceState, DeviceStateListener},
        RetroController,
    },
    retro_core::{option_manager::OptionManager, test_tools},
    thread_stack::main_stack::{SaveImg, SavePath},
    tinic_super::{core_info::CoreInfo, core_info_helper::CoreInfoHelper},
};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};

lazy_static! {
    static ref DEVICE_STATE_LISTENER: RwLock<DeviceStateListener> = RwLock::new(|_, _| {});
    static ref CHANNEL: Arc<ThreadChannel> = Arc::new(ThreadChannel::new());
}

pub struct Tinic {
    pub controller: Arc<Mutex<RetroController>>,
    game_thread: GameThread,
    pub core_options: Option<Arc<OptionManager>>,
    retro_paths: Option<RetroPaths>,
}

impl Tinic {
    pub fn new(listener: DeviceStateListener) -> Result<Tinic, ErroHandle> {
        match DEVICE_STATE_LISTENER.write() {
            Ok(mut device_listener) => *device_listener = listener,
            Err(e) => {
                return Err(ErroHandle {
                    level: RETRO_LOG_ERROR,
                    message: e.to_string(),
                })
            }
        }

        let controller_ctx = Arc::new(Mutex::new(RetroController::new(
            Tinic::device_state_listener,
        )?));

        Ok(Self {
            game_thread: GameThread::new(controller_ctx.clone()),
            core_options: None,
            controller: controller_ctx,
            retro_paths: None,
        })
    }

    pub fn set_tinic_dir(&mut self, retro_paths: RetroPaths) {
        self.retro_paths.replace(retro_paths);
    }

    pub async fn load_game(&mut self, core_path: &str, rom_path: &str) -> Result<bool, ErroHandle> {
        self.game_thread.start(CHANNEL.get_notify())?;

        let core_options = CHANNEL
            .load_game(core_path, rom_path, self.try_get_retro_path()?.clone())
            .await;

        self.core_options = core_options;

        Ok(self.core_options.is_some())
    }

    pub fn pause(&self) {
        CHANNEL.pause_game();
    }

    pub fn resume(&self) {
        CHANNEL.resume_game();
    }

    pub async fn save_state(&self, slot: usize) -> Option<(SavePath, SaveImg)> {
        CHANNEL.save_state(slot).await
    }

    pub async fn load_state(&self, slot: usize) -> bool {
        CHANNEL.load_state(slot).await
    }

    pub fn connect_device(device: Device) {
        CHANNEL.connect_device(device);
    }

    pub fn reset(&self) {
        CHANNEL.reset_game();
    }

    pub fn quit(&mut self) {
        self.core_options.take();
        self.game_thread.stop();
    }

    pub fn enable_full_screen(&self) {
        CHANNEL.enable_full_screen();
    }

    pub fn disable_full_screen(&self) {
        CHANNEL.disable_full_screen();
    }

    pub async fn try_update_core_infos(&mut self, force_update: bool) -> Result<(), ErroHandle> {
        match CoreInfoHelper::try_update_core_infos(self.try_get_retro_path()?, force_update).await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(ErroHandle {
                level: RETRO_LOG_ERROR,
                message: e.to_string(),
            }),
        }
    }

    pub fn get_cores_infos(&mut self) -> Result<Vec<CoreInfo>, ErroHandle> {
        Ok(CoreInfoHelper::get_core_infos(
            &self.try_get_retro_path()?.infos.clone().to_owned(),
        ))
    }

    pub fn get_compatibility_info_cores(&self, rom: &String) -> Vec<CoreInfo> {
        CoreInfoHelper::get_compatibility_core_infos(PathBuf::from(rom))
    }
}

impl Tinic {
    fn device_state_listener(state: DeviceState, device: Device) {
        if let Ok(listener) = DEVICE_STATE_LISTENER.read() {
            match &state {
                DeviceState::Connected | DeviceState::Disconnected => {
                    CHANNEL.connect_device(device.clone());
                }
                _ => {}
            }
            listener(state, device);
        };
    }

    fn try_get_retro_path(&mut self) -> Result<&RetroPaths, ErroHandle> {
        let retro_paths = &mut self.retro_paths;

        if let Some(path) = retro_paths {
            return Ok(path);
        }

        let path = test_tools::paths::get_paths()?;
        retro_paths.replace(path.clone());

        return if let Some(path) = retro_paths {
            Ok(path)
        } else {
            Err(ErroHandle {
                level: RETRO_LOG_ERROR,
                message: "retro_path nao foi definido".to_string(),
            })
        };
    }
}
