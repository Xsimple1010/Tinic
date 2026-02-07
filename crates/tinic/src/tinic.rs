use crate::{
    game_thread::game_thread_handle::GameThread,
    generics::{erro_handle::ErroHandle, retro_paths::RetroPaths, types::TMutex},
    libretro_sys::binding_libretro::retro_log_level::RETRO_LOG_ERROR,
    retro_controllers::{
        devices_manager::{Device, DeviceListener},
        RetroController,
    },
    retro_core::{option_manager::OptionManager, test_tools},
    thread_stack::main_stack::{SaveImg, SavePath},
    tinic_super::{core_info::CoreInfo, core_info_helper::CoreInfoHelper},
};
use std::path::PathBuf;
use std::sync::Arc;

pub struct Tinic {
    pub controller: Arc<TMutex<RetroController>>,
    pub core_options: Option<Arc<OptionManager>>,
    game_thread: Arc<GameThread>,
    retro_paths: Option<RetroPaths>,
}

impl Drop for Tinic {
    fn drop(&mut self) {
        let _ = self.quit();
    }
}

impl Tinic {
    pub fn new(listener: Box<dyn DeviceListener>) -> Result<Tinic, ErroHandle> {
        let game_thread = Arc::new(GameThread::new());
        let tinic_device_handle = TinicDeviceHandle {
            game_thread: game_thread.clone(),
            extern_listener: listener,
        };
        let controller = TMutex::new(RetroController::new(Box::new(tinic_device_handle))?);

        Ok(Self {
            game_thread,
            core_options: None,
            controller,
            retro_paths: None,
        })
    }

    pub fn set_tinic_dir(&mut self, retro_paths: RetroPaths) {
        self.retro_paths.replace(retro_paths);
    }

    pub async fn load_game(&mut self, core_path: &str, rom_path: &str) -> Result<bool, ErroHandle> {
        let retro_path = self.try_get_retro_path()?.clone();
        self.game_thread.start(self.controller.clone())?;

        let core_options = self
            .game_thread
            .channel
            .load_game(core_path, rom_path, retro_path)
            .await;

        self.core_options = core_options;

        Ok(self.core_options.is_some())
    }

    pub fn pause(&self) {
        self.game_thread.channel.pause_game();
    }

    pub fn resume(&self) {
        self.game_thread.channel.resume_game();
    }

    pub async fn save_state(&self, slot: usize) -> Option<(SavePath, SaveImg)> {
        self.game_thread.channel.save_state(slot).await
    }

    pub async fn load_state(&self, slot: usize) -> bool {
        self.game_thread.channel.load_state(slot).await
    }

    pub fn connect_device(&self, device: Device) {
        self.game_thread.channel.connect_device(device);
    }

    pub fn reset(&self) {
        self.game_thread.channel.reset_game();
    }

    pub async fn quit(&mut self) -> bool {
        if self.game_thread.is_running() {
            self.core_options.take();
            self.game_thread.channel.quit().await
        } else {
            true
        }
    }

    pub fn enable_full_screen(&self) {
        self.game_thread.channel.enable_full_screen();
    }

    pub fn disable_full_screen(&self) {
        self.game_thread.channel.disable_full_screen();
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
    fn try_get_retro_path(&mut self) -> Result<&RetroPaths, ErroHandle> {
        let retro_paths = &mut self.retro_paths;

        if let Some(path) = retro_paths {
            return Ok(path);
        }

        let path = test_tools::paths::get_paths()?;
        retro_paths.replace(path.clone());

        if let Some(path) = retro_paths {
            Ok(path)
        } else {
            Err(ErroHandle {
                level: RETRO_LOG_ERROR,
                message: "retro_path nao foi definido".to_string(),
            })
        }
    }
}

#[derive(Debug)]
struct TinicDeviceHandle {
    game_thread: Arc<GameThread>,
    extern_listener: Box<dyn DeviceListener>,
}

impl DeviceListener for TinicDeviceHandle {
    fn connected(&self, device: Device) {
        self.game_thread.channel.connect_device(device.clone());
        self.extern_listener.connected(device);
    }

    fn disconnected(&self, device: Device) {
        self.extern_listener.disconnected(device);
    }

    fn button_pressed(&self, button: String, device: Device) {
        self.extern_listener.button_pressed(button, device);
    }
}
