use libretro_sys::binding_libretro::retro_log_level as RetroLogLevel;

#[derive(Debug)]
pub struct ErroHandle {
    pub level: RetroLogLevel,
    pub message: String,
}
