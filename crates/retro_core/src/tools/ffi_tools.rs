use generics::erro_handle::ErroHandle;
use libretro_sys::binding_libretro::retro_log_level::RETRO_LOG_ERROR;
use std::ffi::{c_char, CStr, CString};
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

pub fn make_c_string(rs_string: &str) -> Result<CString, ErroHandle> {
    match CString::new(rs_string) {
        Ok(c_string) => Ok(c_string),
        _ => Err(ErroHandle {
            level: RETRO_LOG_ERROR,
            message: "Nao foi possível cria uma c_string".to_string(),
        }),
    }
}
