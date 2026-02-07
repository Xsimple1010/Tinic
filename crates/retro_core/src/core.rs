pub use crate::av_info::{AvInfo, Geometry, Timing, Video};
use crate::core_env::{self, RetroEnvCallbacks};
use crate::graphic_api::GraphicApi;
use crate::retro_context::RetroContext;
use crate::tools::game_tools::RomTools;
use crate::{managers::option_manager::OptionManager, system::System};
use generics::erro_handle::ErroHandle;
use generics::retro_paths::RetroPaths;
pub use libretro_sys::binding_libretro::retro_language;
use libretro_sys::binding_libretro::retro_log_level::RETRO_LOG_ERROR;
pub use libretro_sys::binding_libretro::retro_pixel_format;
use libretro_sys::binding_libretro::LibretroRaw;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub type CoreWrapperIns = Arc<CoreWrapper>;

pub struct CoreWrapper {
    /// # Retro_context_associated
    ///
    /// Adicionei isso com o proposito de chamar futuras callbacks que serão adicionadas
    /// [RetroContext] dentro das callbacks fornecidas por [environment],
    pub retro_ctx_associated: Uuid,
    pub rom_name: RwLock<String>,
    pub initialized: AtomicBool,
    pub game_loaded: AtomicBool,
    pub support_no_game: AtomicBool,
    pub av_info: Arc<AvInfo>,
    pub system: System,
    pub paths: RetroPaths,
    pub options: Arc<OptionManager>,
    raw: Arc<LibretroRaw>,
    pub callbacks: RetroEnvCallbacks,
}

impl CoreWrapper {
    pub fn new(
        retro_ctx_associated: Uuid,
        core_path: &str,
        paths: RetroPaths,
        callbacks: RetroEnvCallbacks,
        graphic_api: GraphicApi,
    ) -> Result<CoreWrapperIns, ErroHandle> {
        let raw = unsafe { LibretroRaw::new(core_path).unwrap() };

        let system = System::new(&raw);

        let options = Arc::new(OptionManager::new(
            &paths.opt,
            system.info.library_name.clone().to_string(),
        ));

        let core = Arc::new(CoreWrapper {
            raw: Arc::new(raw),
            initialized: AtomicBool::new(false),
            game_loaded: AtomicBool::new(false),
            support_no_game: AtomicBool::new(false),
            av_info: Arc::new(AvInfo::new(graphic_api)),
            rom_name: RwLock::new("".to_string()),
            system,
            paths,
            options,
            callbacks,
            retro_ctx_associated,
        });

        core_env::configure(core.clone());

        unsafe {
            core.raw
                .retro_set_environment(Some(core_env::core_environment));

            core.raw
                .retro_set_audio_sample(Some(core_env::audio_sample_callback));

            core.raw
                .retro_set_audio_sample_batch(Some(core_env::audio_sample_batch_callback));

            core.raw
                .retro_set_video_refresh(Some(core_env::video_refresh_callback));

            core.raw
                .retro_set_input_poll(Some(core_env::input_poll_callback));

            core.raw
                .retro_set_input_state(Some(core_env::input_state_callback));
        }

        println!("CORE INICIALIZADO");

        Ok(core)
    }

    pub fn init(&self) -> Result<(), ErroHandle> {
        if self.game_loaded.load(Ordering::SeqCst) || self.initialized.load(Ordering::SeqCst) {
            return Err(ErroHandle {
                level: RETRO_LOG_ERROR,
                message: "Para inicializar um novo núcleo e necessário descarrega o núcleo atual"
                    .to_string(),
            });
        }

        unsafe {
            self.initialized.store(true, Ordering::SeqCst);
            self.raw.retro_init();

            Ok(())
        }
    }

    pub fn load_game(&self, path: &str) -> Result<Arc<AvInfo>, ErroHandle> {
        if self.game_loaded.load(Ordering::SeqCst) {
            return Err(ErroHandle {
                level: RETRO_LOG_ERROR,
                message: "Ja existe uma rom carregada no momento".to_string(),
            });
        }

        if !self.initialized.load(Ordering::SeqCst) {
            return Err(ErroHandle {
                level: RETRO_LOG_ERROR,
                message: "Para carregar uma rom o núcleo deve esta inicializado".to_string(),
            });
        }

        let loaded = RomTools::try_load_game(&self.raw, &self.system.info, path)?;

        if loaded {
            *self.rom_name.write().unwrap() = RomTools::get_rom_name(&PathBuf::from(path))?;

            self.av_info.update_av_info(&self.raw);

            Ok(self.av_info.clone())
        } else {
            Err(ErroHandle {
                level: RETRO_LOG_ERROR,
                message: "nao foi possível carregar a rom".to_string(),
            })
        }
    }

    pub fn reset(&self) -> Result<(), ErroHandle> {
        if !self.initialized.load(Ordering::SeqCst) {
            return Err(ErroHandle {
                level: RETRO_LOG_ERROR,
                message: "O núcleo nao foi inicializado".to_string(),
            });
        }

        if !self.game_loaded.load(Ordering::SeqCst) {
            return Err(ErroHandle {
                level: RETRO_LOG_ERROR,
                message: "Nao ha nenhuma rum carregada no momento".to_string(),
            });
        }

        unsafe {
            self.raw.retro_reset();
        }

        Ok(())
    }

    pub fn run(&self) -> Result<(), ErroHandle> {
        if !self.initialized.load(Ordering::SeqCst) {
            return Err(ErroHandle {
                level: RETRO_LOG_ERROR,
                message: "O núcleo nao foi inicializado".to_string(),
            });
        }

        if !self.game_loaded.load(Ordering::SeqCst) {
            return Err(ErroHandle {
                level: RETRO_LOG_ERROR,
                message: "Nao ha nenhuma rum carregada no momento".to_string(),
            });
        }

        println!("TENTA CRIAR OS NOVOS BUFFERS AV");

        unsafe { self.raw.retro_run() }

        Ok(())
    }

    pub fn de_init(&self) -> Result<(), ErroHandle> {
        //Se uma *rom* estive carrega ela deve ser descarregada primeiro
        match self.unload_game() {
            Ok(..) => {}
            Err(e) => match &e.level {
                RETRO_LOG_ERROR => {}
                _ => {
                    unsafe {
                        self.raw.retro_deinit();
                    }
                    self.initialized.store(false, Ordering::SeqCst);
                    core_env::delete_local_core_ctx();

                    return Err(e);
                }
            },
        }

        unsafe {
            self.raw.retro_deinit();
        }
        self.initialized.store(false, Ordering::SeqCst);
        core_env::delete_local_core_ctx();

        Ok(())
    }

    pub fn connect_controller(&self, port: u32, controller: u32) -> Result<(), ErroHandle> {
        if !self.initialized.load(Ordering::SeqCst) {
            return Err(ErroHandle {
                level: RETRO_LOG_ERROR,
                message: "Nao é possível conectar um controle pois nenhum núcleo foi inicializado"
                    .to_string(),
            });
        }

        unsafe {
            self.raw.retro_set_controller_port_device(port, controller);
        }

        Ok(())
    }

    pub fn unload_game(&self) -> Result<(), ErroHandle> {
        if !self.game_loaded.load(Ordering::SeqCst) {
            return Err(ErroHandle {
                level: RETRO_LOG_ERROR,
                message: "A rom ja foi descarregada anteriormente".to_string(),
            });
        }

        if !self.initialized.load(Ordering::SeqCst) {
            return Err(ErroHandle {
                level: RETRO_LOG_ERROR,
                message: "Para descarregar uma rom o núcleo deve esta inicializado".to_string(),
            });
        }

        unsafe {
            self.raw.retro_unload_game();
        }
        self.game_loaded.store(false, Ordering::SeqCst);

        Ok(())
    }

    pub fn save_state(&self, slot: usize) -> Result<PathBuf, ErroHandle> {
        if !self.game_loaded.load(Ordering::SeqCst) {
            return Err(ErroHandle {
                level: RETRO_LOG_ERROR,
                message: "Uma rom precisa ser carregada primeiro".to_string(),
            });
        }

        if !self.initialized.load(Ordering::SeqCst) {
            return Err(ErroHandle {
                level: RETRO_LOG_ERROR,
                message: "Para salva um state o núcleo deve esta inicializado".to_string(),
            });
        }

        RomTools::create_save_state(
            &self.raw,
            &self.paths.save,
            &self.system.info,
            &self.rom_name.read().unwrap(),
            slot,
        )
    }

    pub fn load_state(&self, slot: usize) -> Result<(), ErroHandle> {
        if !self.game_loaded.load(Ordering::SeqCst) {
            return Err(ErroHandle {
                level: RETRO_LOG_ERROR,
                message: "Uma rom precisa ser carregada primeiro".to_string(),
            });
        }

        if !self.initialized.load(Ordering::SeqCst) {
            return Err(ErroHandle {
                level: RETRO_LOG_ERROR,
                message: "Para carregar um state o núcleo deve esta inicializado".to_string(),
            });
        }

        RomTools::load_save_state(
            &self.raw,
            &self.paths.save,
            &self.system.info,
            &self.rom_name.read().unwrap(),
            slot,
        )?;

        Ok(())
    }

    pub fn force_stop(&self) {
        let retro_ctx = RetroContext::get_from_id(&self.retro_ctx_associated)
            .expect("não foi possível forca o fechamento");

        retro_ctx.delete().unwrap();
        core_env::delete_local_core_ctx();
    }
}

#[cfg(test)]
mod core {}
