use std::cell::UnsafeCell;
use std::ffi::{c_uint, c_void};
use std::ptr::null;

pub struct RawTextureData {
    pub data: UnsafeCell<*const c_void>,
    pub width: c_uint,
    pub height: c_uint,
    pub pitch: usize,
}

impl RawTextureData {
    pub fn new() -> Self {
        Self {
            data: UnsafeCell::new(null()),
            pitch: 0,
            height: 0,
            width: 0,
        }
    }
}