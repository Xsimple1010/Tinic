#[doc = "use para evitar o auto consumo de CPU pelas thread secundarias"]
pub const THREAD_SLEEP_TIME: u64 = 16;
pub const MAX_TIME_TO_AWAIT_THREAD_RESPONSE: u64 = 3;

//Core
pub const MAX_CORE_OPTIONS: usize = 90;
pub const MAX_CORE_CONTROLLER_INFO_TYPES: usize = 10;
pub const MAX_CORE_SUBSYSTEM_INFO: usize = 40;
pub const MAX_CORE_SUBSYSTEM_ROM_INFO: usize = 40;
pub const CORE_OPTION_EXTENSION_FILE: &str = ".opt";
pub const DEFAULT_MAX_PORT: usize = 2;
pub const INVALID_CONTROLLER_PORT: i16 = -1;
pub const SAVE_IMAGE_EXTENSION_FILE: &str = "png";
