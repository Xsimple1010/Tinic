use crate::{
    constants::{MAX_CORE_SUBSYSTEM_INFO, MAX_CORE_SUBSYSTEM_ROM_INFO},
    controller_info::ControllerInfo,
    tools::{ffi_tools::get_str_from_ptr, mutex_tools::get_string_rwlock_from_ptr},
};
use libretro_sys::binding_libretro::{
    retro_subsystem_info, retro_subsystem_memory_info, retro_subsystem_rom_info, retro_system_info,
    LibretroRaw,
};
use std::sync::RwLock;

#[derive(Default, Debug)]
pub struct SysInfo {
    pub library_name: RwLock<String>,
    pub library_version: RwLock<String>,
    pub valid_extensions: RwLock<String>,
    pub need_full_path: RwLock<bool>,
    pub block_extract: RwLock<bool>,
}

#[derive(Default, Debug)]
pub struct MemoryInfo {
    pub extension: RwLock<String>,
    pub type_: RwLock<u32>,
}

#[derive(Default, Debug)]
pub struct SubSystemRomInfo {
    pub desc: RwLock<String>,
    pub valid_extensions: RwLock<String>,
    pub need_full_path: RwLock<bool>,
    pub block_extract: RwLock<bool>,
    pub required: RwLock<bool>,
    pub memory: MemoryInfo,
    pub num_memory: RwLock<u32>,
}

#[derive(Default, Debug)]
pub struct SubSystemInfo {
    pub id: RwLock<u32>,
    pub desc: RwLock<String>,
    pub ident: RwLock<String>,
    pub roms: RwLock<Vec<SubSystemRomInfo>>,
}

#[derive(Debug)]
pub struct System {
    pub ports: RwLock<Vec<ControllerInfo>>,
    pub info: SysInfo,
    pub subsystem: RwLock<Vec<SubSystemInfo>>,
}

impl System {
    pub fn new(raw: &LibretroRaw) -> Self {
        unsafe {
            let sys_info = &mut retro_system_info {
                block_extract: false,
                need_fullpath: false,
                library_name: "".as_ptr() as *const i8,
                library_version: "".as_ptr() as *const i8,
                valid_extensions: "".as_ptr() as *const i8,
            };

            raw.retro_get_system_info(sys_info);

            System {
                ports: RwLock::new(Vec::new()),
                subsystem: RwLock::new(Vec::new()),
                info: SysInfo {
                    library_name: RwLock::new(get_str_from_ptr(sys_info.library_name)),
                    library_version: RwLock::new(get_str_from_ptr(sys_info.library_version)),
                    valid_extensions: RwLock::new(get_str_from_ptr(sys_info.valid_extensions)),
                    need_full_path: RwLock::new(sys_info.need_fullpath),
                    block_extract: RwLock::new(sys_info.block_extract),
                },
            }
        }
    }

    pub fn get_subsystem(&self, raw_subsystem: [retro_subsystem_info; MAX_CORE_SUBSYSTEM_INFO]) {
        for raw_sys in raw_subsystem {
            if !raw_sys.ident.is_null() {
                let subsystem = SubSystemInfo::default();

                *subsystem.id.write().unwrap() = raw_sys.id;
                *subsystem.desc.write().unwrap() = get_str_from_ptr(raw_sys.desc);
                *subsystem.ident.write().unwrap() = get_str_from_ptr(raw_sys.ident);

                let roms = unsafe {
                    *(raw_sys.roms as *mut [retro_subsystem_rom_info; MAX_CORE_SUBSYSTEM_ROM_INFO])
                };

                for index in 0..raw_sys.num_roms {
                    let rom = roms[index as usize];

                    let memory = unsafe { *(rom.memory as *mut retro_subsystem_memory_info) };

                    subsystem.roms.write().unwrap().push(SubSystemRomInfo {
                        desc: get_string_rwlock_from_ptr(rom.desc),
                        valid_extensions: get_string_rwlock_from_ptr(rom.valid_extensions),
                        need_full_path: RwLock::new(rom.need_fullpath),
                        block_extract: RwLock::new(rom.block_extract),
                        required: RwLock::new(rom.required),
                        num_memory: RwLock::new(rom.num_memory),
                        memory: MemoryInfo {
                            extension: get_string_rwlock_from_ptr(memory.extension),
                            type_: RwLock::new(memory.type_),
                        },
                    });
                }

                self.subsystem.write().unwrap().push(subsystem);
            } else {
                break;
            }
        }
    }
}

//
#[cfg(test)]
mod test_system {
    use crate::{system::System, test_tools};

    #[test]
    fn test_get_sys_info() {
        let core = test_tools::core::get_core_wrapper();

        let sys = System::new(&core.raw);

        assert_eq!(
            *sys.info.library_name.read().unwrap().clone(),
            "Snes9x".to_owned()
        );

        assert_eq!(
            *sys.info.library_version.read().unwrap().clone(),
            "1.62.3 46f8a6b".to_owned()
        );

        assert_eq!(
            *sys.info.valid_extensions.read().unwrap().clone(),
            "smc|sfc|swc|fig|bs|st".to_owned()
        );

        assert_eq!(*sys.info.block_extract.read().unwrap(), false);

        assert_eq!(*sys.info.need_full_path.read().unwrap(), false);
    }
}
