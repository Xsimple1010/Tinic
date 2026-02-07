use std::sync::{ RwLock};

use super::ffi_tools::get_str_from_ptr;

pub fn get_string_rwlock_from_ptr(ptr: *const i8) -> RwLock<String> {
    RwLock::new(get_str_from_ptr(ptr))
}
