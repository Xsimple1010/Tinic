use crate::{
    binding::binding_libretro::{retro_controller_description, retro_controller_info},
    constants::MAX_CORE_CONTROLLER_INFO_TYPES,
    tools::ffi_tools::get_str_from_ptr,
};
use std::sync::Mutex;

#[derive(Default, Debug)]
pub struct ControllerDescription {
    pub desc: Mutex<String>,
    pub id: Mutex<u32>,
}

#[derive(Default, Debug)]
pub struct ControllerInfo {
    pub types: Vec<ControllerDescription>,
    pub num_types: Mutex<u32>,
}

impl ControllerInfo {
    pub fn from_raw(raw_ctr_info: retro_controller_info) -> ControllerInfo {
        let mut controller_info = ControllerInfo::default();

        *controller_info.num_types.lock().unwrap() = raw_ctr_info.num_types;

        let ctr_types = unsafe {
            *(raw_ctr_info.types
                as *mut [retro_controller_description; MAX_CORE_CONTROLLER_INFO_TYPES])
        };

        for index in 0..raw_ctr_info.num_types {
            let raw_ctr_type = ctr_types[index as usize];

            if !raw_ctr_type.desc.is_null() {
                let desc = ControllerDescription {
                    desc: Mutex::new(get_str_from_ptr(raw_ctr_type.desc)),
                    id: Mutex::new(raw_ctr_type.id),
                };

                controller_info.types.push(desc);
            }
        }

        controller_info
    }
}
