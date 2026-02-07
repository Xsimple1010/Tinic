use crate::av_info::AvInfo;
use crate::core_env::{self, RetroEnvCallbacks};
use crate::graphic_api::GraphicApi;
use crate::tools::game_tools::{RomTools, SaveInfo};
use crate::{managers::option_manager::OptionManager, system::System};
use generics::constants::INVALID_CONTROLLER_PORT;
use generics::erro_handle::ErroHandle;
use generics::retro_paths::RetroPaths;
use libretro_sys::binding_libretro::LibretroRaw;
use std::ffi::c_void;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

pub type RetroCoreIns = Rc<RetroCore>;

pub struct RetroCore {
    pub rom_name: RwLock<String>,
    pub initialized: AtomicBool,
    pub game_loaded: AtomicBool,
    pub support_no_game: AtomicBool,
    pub av_info: Arc<AvInfo>,
    pub system: System,
    pub paths: RetroPaths,
    pub options: Arc<OptionManager>,
    pub callbacks: RetroEnvCallbacks,
    raw: Arc<LibretroRaw>,
}

impl RetroCore {
    pub fn new(
        core_path: &str,
        paths: RetroPaths,
        callbacks: RetroEnvCallbacks,
        graphic_api: GraphicApi,
    ) -> Result<RetroCoreIns, ErroHandle> {
        let raw = unsafe { LibretroRaw::new(core_path).unwrap() };

        let system = System::new(&raw);

        let options = Arc::new(OptionManager::new(
            &paths.opt,
            system.info.library_name.clone().to_string(),
        ));

        let core = Rc::new(RetroCore {
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

        core.init()?;

        Ok(core)
    }

    fn init(&self) -> Result<(), ErroHandle> {
        if self.game_loaded.load(Ordering::SeqCst) || self.initialized.load(Ordering::SeqCst) {
            return Err(ErroHandle::new(
                "Para inicializar um novo núcleo e necessário descarrega o núcleo atual",
            ));
        }

        unsafe {
            self.initialized.store(true, Ordering::SeqCst);
            self.raw.retro_init();

            Ok(())
        }
    }

    pub fn load_game(&self, path: &str) -> Result<Arc<AvInfo>, ErroHandle> {
        if self.game_loaded.load(Ordering::SeqCst) {
            return Err(ErroHandle::new("Ja existe uma rom carregada no momento"));
        }

        if !self.initialized.load(Ordering::SeqCst) {
            return Err(ErroHandle::new(
                "Para carregar uma rom o núcleo deve esta inicializado",
            ));
        }

        let loaded = RomTools::try_load_game(&self.raw, &self.system.info, path)?;
        self.game_loaded.store(loaded, Ordering::SeqCst);

        if loaded {
            *self.rom_name.write()? = RomTools::get_rom_name(&PathBuf::from(path))?;

            self.av_info.update_av_info(&self.raw)?;

            Ok(self.av_info.clone())
        } else {
            Err(ErroHandle::new("nao foi possível carregar a rom"))
        }
    }

    pub fn reset(&self) -> Result<(), ErroHandle> {
        if !self.initialized.load(Ordering::SeqCst) {
            return Err(ErroHandle::new("O núcleo nao foi inicializado"));
        }

        if !self.game_loaded.load(Ordering::SeqCst) {
            return Err(ErroHandle::new("Nao ha nenhuma rum carregada no momento"));
        }

        unsafe {
            self.raw.retro_reset();
        }

        Ok(())
    }

    pub fn run(&self) -> Result<(), ErroHandle> {
        if !self.initialized.load(Ordering::SeqCst) {
            return Err(ErroHandle::new("O núcleo nao foi inicializado"));
        }

        if !self.game_loaded.load(Ordering::SeqCst) {
            return Err(ErroHandle::new("Nao ha nenhuma rum carregada no momento"));
        }

        unsafe { self.raw.retro_run() }

        Ok(())
    }

    pub fn de_init(&self) -> Result<(), ErroHandle> {
        //Se uma *rom* estive carrega ela deve ser descarregada primeiro
        if let Err(e) = self.unload_game() {
            self.initialized.store(false, Ordering::SeqCst);
            core_env::delete_local_core_ctx();

            return Err(e);
        }

        unsafe {
            self.raw.retro_deinit();
        }
        self.initialized.store(false, Ordering::SeqCst);
        core_env::delete_local_core_ctx();

        Ok(())
    }

    pub fn connect_controller(&self, port: i16, controller: u32) -> Result<(), ErroHandle> {
        if !self.initialized.load(Ordering::SeqCst) {
            return Err(ErroHandle::new(
                "Nao é possível conectar um controle pois nenhum núcleo foi inicializado",
            ));
        }

        if port != INVALID_CONTROLLER_PORT {
            unsafe {
                self.raw
                    .retro_set_controller_port_device(port as u32, controller);
            }
        }

        Ok(())
    }

    pub fn unload_game(&self) -> Result<(), ErroHandle> {
        if !self.game_loaded.load(Ordering::SeqCst) {
            return Ok(());
        }

        if !self.initialized.load(Ordering::SeqCst) {
            return Err(ErroHandle::new(
                "Para descarregar uma rom o núcleo deve esta inicializado",
            ));
        }

        unsafe {
            self.raw.retro_unload_game();
        }
        self.game_loaded.store(false, Ordering::SeqCst);

        Ok(())
    }

    pub fn save_state(&self, slot: usize) -> Result<PathBuf, ErroHandle> {
        if !self.game_loaded.load(Ordering::SeqCst) {
            return Err(ErroHandle::new("Uma rom precisa ser carregada primeiro"));
        }

        if !self.initialized.load(Ordering::SeqCst) {
            return Err(ErroHandle::new(
                "Para salva um state o núcleo deve esta inicializado",
            ));
        }

        RomTools::create_save_state(
            SaveInfo {
                slot,
                library_name: &self.system.info.library_name,
                rom_name: &*self.rom_name.read()?,
                save_dir: &self.paths.save,
                buffer_size: unsafe { self.raw.retro_serialize_size() },
            },
            |data, size| unsafe {
                self.raw
                    .retro_serialize(data.as_mut_ptr() as *mut c_void, size)
            },
        )
    }

    pub fn load_state(&self, slot: usize) -> Result<(), ErroHandle> {
        if !self.game_loaded.load(Ordering::SeqCst) {
            return Err(ErroHandle::new("Uma rom precisa ser carregada primeiro"));
        }

        if !self.initialized.load(Ordering::SeqCst) {
            return Err(ErroHandle::new(
                "Para carregar um state o núcleo deve esta inicializado",
            ));
        }

        RomTools::load_save_state(
            SaveInfo {
                slot,
                library_name: &self.system.info.library_name,
                rom_name: &*self.rom_name.read()?,
                save_dir: &self.paths.save,
                buffer_size: unsafe { self.raw.retro_serialize_size() },
            },
            |data, size| unsafe {
                self.raw
                    .retro_unserialize(data.as_mut_ptr() as *mut c_void, size)
            },
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod core {}
