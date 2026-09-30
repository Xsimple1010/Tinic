use crate::app::listener::{GameState, WindowState};
use crate::{SaveStateInfo, TinicGameInfo, WindowListener};
use retro_audio::RetroAudio;
use retro_controllers::{RetroController, RetroGamePad};
use retro_core::{RetroCore, RetroCoreIns, RetroEnvCallbacks, graphic_api::GraphicApi};
use retro_video::RetroVideo;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::{path::Path, sync::Arc};
use tinic_generics::error_handle::TinicResult;
use tinic_generics::retro_paths::RetroPaths;
use tinic_generics::{constants::SAVE_IMAGE_EXTENSION_FILE, error_handle::ErrorHandle};
use winit::dpi::PhysicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::PhysicalKey;

pub struct TinicGameCtx {
    retro_video: RetroVideo,
    retro_audio: RetroAudio,
    retro_core: RetroCoreIns,
    can_request_new_frames: bool,
    rom_path: String,
    pub controller: Arc<RetroController>,
    window_listener: Arc<Box<dyn WindowListener>>,
}

impl TinicGameCtx {
    pub fn new(
        game_info: TinicGameInfo,
        controller: Arc<RetroController>,
        window_listener: Arc<Box<dyn WindowListener>>,
    ) -> Result<Self, ErrorHandle> {
        let retro_video = RetroVideo::default();
        let retro_audio = RetroAudio::new()?;

        let callbacks = RetroEnvCallbacks {
            audio: Box::new(retro_audio.get_core_cb()),
            video: Box::new(retro_video.get_core_cb()?),
            controller: Box::new(controller.get_core_cb()),
        };

        let paths = RetroPaths::from_base(game_info.sys_dir)?;

        let retro_core = RetroCore::new(
            &game_info.core.into(),
            paths,
            callbacks,
            GraphicApi::with_opengl(),
        )?;

        let game_pads = controller.get_list()?;

        if game_pads.len().eq(&0) {
            let keyboard = controller.active_keyboard();
            retro_core.connect_controller(keyboard.retro_port, keyboard.retro_type)?;
        } else {
            for game_pad in game_pads {
                retro_core.connect_controller(game_pad.retro_port, game_pad.retro_type)?;
            }
        }

        Ok(Self {
            retro_video,
            retro_audio,
            retro_core,
            controller,
            rom_path: game_info.rom,
            window_listener,
            can_request_new_frames: true,
        })
    }

    pub fn resize_window(&mut self, size: PhysicalSize<u32>) -> TinicResult<()> {
        self.retro_video.resize_window(size.width, size.height)
    }

    pub fn toggle_keyboard_usage(&self) -> TinicResult<()> {
        if self.controller.is_using_keyboard() {
            self.disable_keyboard();
            Ok(())
        } else {
            self.active_keyboard()
        }
    }

    pub fn disable_keyboard(&self) {
        self.controller.disable_keyboard();
        self.window_listener.keyboard_state(false)
    }

    pub fn active_keyboard(&self) -> TinicResult<()> {
        let keyboard = self.controller.active_keyboard();
        self.window_listener.keyboard_state(true);

        self.retro_core
            .connect_controller(keyboard.retro_port, keyboard.retro_type)
    }

    pub fn update_keyboard_state(&self, native: PhysicalKey, pressed: bool) {
        self.controller.update_keyboard(native, pressed)
    }

    pub fn create_window(&mut self, event_loop: &ActiveEventLoop) -> TinicResult<()> {
        self.retro_video
            .create_window(&self.retro_core.av_info, event_loop)
            .map_err(|e| {
                self.window_listener.game_state_change(GameState::Closed);
                e
            })
    }

    pub fn init_core(&mut self) -> TinicResult<()> {
        let err_handle = |e: ErrorHandle| {
            self.window_listener.game_state_change(GameState::Closed);
            e
        };

        if self.retro_core.game_loaded.load(Ordering::Relaxed) {
            return Ok(());
        }

        self.can_request_new_frames = false;
        self.retro_video.create_draw_context().map_err(err_handle)?;

        self.retro_core
            .load_game(&self.rom_path)
            .map_err(err_handle)?;

        self.retro_video
            .init_frame_buffer(&self.retro_core.av_info)
            .map_err(err_handle)?;

        self.retro_audio
            .init(&self.retro_core.av_info)
            .map_err(err_handle)?;

        self.can_request_new_frames = true;
        self.controller.stop_thread_events();
        self.window_listener.game_state_change(GameState::Running);
        self.window_listener
            .window_state_change(WindowState::Opened);

        Ok(())
    }

    pub fn suspend_window(&mut self) {
        let _ = self.retro_video.teardown_graphics();
        self.retro_audio.stop();
        self.controller.resume_thread_events();

        self.window_listener
            .window_state_change(WindowState::Closed);
    }

    pub fn destroy_retro_ctx(&mut self) -> TinicResult<()> {
        self.retro_core.de_init()?;

        self.retro_core
            .av_info
            .video
            .graphic_api
            .try_destroy_ctx()?;
        self.retro_video.teardown_graphics()?;

        self.retro_audio.stop();
        self.controller.resume_thread_events();

        self.window_listener.game_state_change(GameState::Closed);
        self.window_listener
            .window_state_change(WindowState::Closed);
        self.can_request_new_frames = false;

        Ok(())
    }

    pub fn redraw_request(&self) -> TinicResult<()> {
        if self.can_request_new_frames {
            self.retro_video.request_redraw()
        } else {
            Ok(())
        }
    }

    pub fn draw_new_frame(&mut self) -> TinicResult<()> {
        if !self.can_request_new_frames {
            return Ok(());
        }

        if !self.retro_core.game_loaded.load(Ordering::SeqCst) {
            return Ok(());
        }

        self.retro_video
            .sync
            .prepare_sync(&self.retro_core.av_info)?;
        self.retro_video.prepare_to_core()?;
        self.retro_core.run()?;
        self.retro_video.draw_new_frame()?;
        self.retro_video.sync.sync_now()?;
        Ok(())
    }

    pub fn reset(&self) -> TinicResult<()> {
        self.retro_core.reset()
    }

    pub fn save_state(&self, slot: usize) -> TinicResult<()> {
        // Erros handles
        let err_handle = |e: ErrorHandle| {
            self.window_listener
                .save_state_result(SaveStateInfo::Failed);
            e
        };

        let file_err_handle = |path: PathBuf| -> Result<String, ErrorHandle> {
            Ok(path
                .canonicalize()
                .map_err(|e| err_handle(e.into()))?
                .to_str()
                .ok_or_else(|| {
                    err_handle(ErrorHandle::new("Erro ao converter o caminho para string"))
                })?
                .to_string())
        };
        // =========================================================

        let save_path = self.retro_core.save_state(slot).map_err(err_handle)?;

        let mut img_path = save_path.clone();
        img_path.set_extension(SAVE_IMAGE_EXTENSION_FILE);

        if self.print_screen(&img_path).is_err() {
            self.window_listener
                .save_state_result(SaveStateInfo::Failed);
            return Ok(());
        }

        let save_path = file_err_handle(save_path)?;
        let save_img_preview = file_err_handle(img_path)?;

        self.window_listener
            .save_state_result(SaveStateInfo::Susses {
                save_path,
                save_img_preview,
            });

        Ok(())
    }

    pub fn load_state(&self, slot: usize) -> TinicResult<()> {
        match self.retro_core.load_state(slot) {
            Ok(_) => {
                self.window_listener.load_state_result(true);
                Ok(())
            }
            Err(e) => {
                self.window_listener.load_state_result(false);
                Err(e)
            }
        }
    }

    pub fn print_screen(&self, out_path: &Path) -> TinicResult<()> {
        self.retro_video
            .print_screen(out_path, &self.retro_core.av_info)
    }

    pub fn toggle_full_screen_mode(&mut self) -> TinicResult<()> {
        self.retro_video.toggle_window_mode()
    }

    pub fn toggle_can_request_new_frames(&mut self) -> TinicResult<()> {
        if self.can_request_new_frames {
            self.pause()
        } else {
            self.resume()
        }
    }

    pub fn pause(&mut self) -> TinicResult<()> {
        self.controller.resume_thread_events();
        self.can_request_new_frames = false;
        self.retro_audio.pause()?;
        self.window_listener.game_state_change(GameState::Paused);
        Ok(())
    }

    pub fn resume(&mut self) -> TinicResult<()> {
        self.controller.stop_thread_events();
        self.can_request_new_frames = true;
        self.retro_audio.play()?;
        self.window_listener.game_state_change(GameState::Running);
        Ok(())
    }

    pub fn connect_controller(&self, device: RetroGamePad) -> TinicResult<()> {
        self.retro_core
            .connect_controller(device.retro_port, device.retro_type)
    }
}
