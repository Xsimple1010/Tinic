use crate::error_handle::ErrorHandle;

#[doc = "use para evitar o auto consumo de CPU pelas thread secundarias"]
pub const THREAD_SLEEP_TIME: u64 = 16;
pub const MAX_TIME_TO_AWAIT_THREAD_RESPONSE: u64 = 3;

//Core
pub const MAX_CORE_OPTIONS: usize = 90;
pub const MAX_CORE_CONTROLLER_INFO_TYPES: usize = 10;
pub const MAX_CORE_SUBSYSTEM_INFO: usize = 40;
pub const MAX_CORE_SUBSYSTEM_ROM_INFO: usize = 40;
pub const CORE_OPTION_EXTENSION_FILE: &str = "opt";
pub const DEFAULT_MAX_PORT: usize = 2;
pub const INVALID_CONTROLLER_PORT: i16 = -1;
pub const SAVE_IMAGE_EXTENSION_FILE: &str = "png";
pub const SAVE_EXTENSION_FILE: &str = "save";

//URLS
pub const CORE_INFOS_URL: &str = "https://buildbot.libretro.com/assets/frontend/info.zip";
pub const WINDOWS_CORES_URL: &str =
    "https://buildbot.libretro.com/stable/1.19.1/windows/x86_64/RetroArch_cores.7z";
pub const LINUX_CORES_URL: &str =
    "https://buildbot.libretro.com/stable/1.19.1/linux/x86_64/RetroArch_cores.7z";
pub const RDB_BASE_URL: &str =
    "https://raw.githubusercontent.com/libretro/libretro-database/master/rdb";

// pub const THUMBNAIL_BASE_URL: &str = "https://raw.githubusercontent.com/libretro/libretro-thumbnails/master/Nintendo%20-%20Wii/Named_Boxarts/1%2C000%2C000%20Dollar%20Pyramid%20(USA).png";
pub const THUMBNAIL_BASE_URL: &str =
    "https://raw.githubusercontent.com/libretro/libretro-thumbnails/master";

pub const RDB_HEADER_SIZE: usize = 0x10;

pub fn cores_url() -> Result<&'static str, ErrorHandle> {
    if cfg!(target_os = "windows") {
        Ok(WINDOWS_CORES_URL)
    } else if cfg!(target_os = "linux") {
        Ok(LINUX_CORES_URL)
    } else {
        Err(ErrorHandle {
            message: "Sistema operacional não suportado".to_string(),
        })
    }
}
