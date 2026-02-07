use std::sync::RwLock;

use crate::tools::validation::InputValidator;

pub fn get_string_rwlock_from_ptr(ptr: *const i8) -> RwLock<String> {
    let st = unsafe {
        InputValidator::read_safe_c_string(ptr, 255).unwrap_or_else(|e| {
            println!("Error reading string from pointer: {:?}", e);
            String::new()
        })
    };

    RwLock::new(st)
}
