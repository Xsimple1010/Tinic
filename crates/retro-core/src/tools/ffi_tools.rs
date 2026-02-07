use std::ffi::{CStr, c_char};
use std::sync::Arc;

pub fn get_str_from_ptr(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return "".to_string();
    }

    let c_char_ptr: *mut c_char = ptr as *mut c_char;
    let c_str = unsafe { CStr::from_ptr(c_char_ptr) };
    let str_slice = c_str.to_str().unwrap();

    str::to_owned(str_slice)
}

pub fn get_arc_string_from_ptr(ptr: *const c_char) -> Arc<String> {
    Arc::new(get_str_from_ptr(ptr))
}
