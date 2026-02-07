use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use libretro_sys::binding_libretro::retro_log_level;

use crate::erro_handle::ErroHandle;

pub type ArcTMuxte<T> = Arc<TMutex<T>>;

#[derive(Debug)]
pub struct TMutex<T> {
    value: Mutex<T>,
}

impl<T> TMutex<T> {
    pub fn new(value: T) -> ArcTMuxte<T> {
        Arc::new(Self {
            value: Mutex::new(value),
        })
    }

    pub fn store(&self, value: T) {
        match self.value.lock() {
            Ok(mut v) => *v = value,
            Err(e) => {
                let mut v = e.into_inner();
                *v = value;
            }
        }
    }

    pub fn load_or(&self, or: T) -> MutexGuard<'_, T> {
        match self.value.lock() {
            Ok(v) => v,
            Err(e) => {
                let mut v = e.into_inner();
                *v = or;
                v
            }
        }
    }

    pub fn store_or_else<CA: FnOnce(PoisonError<MutexGuard<'_, T>>)>(&self, value: T, or_else: CA) {
        match self.value.lock() {
            Ok(mut v) => *v = value,
            Err(e) => or_else(e),
        }
    }

    pub fn try_load(&self) -> Result<MutexGuard<'_, T>, ErroHandle> {
        match self.value.lock() {
            Ok(v) => Ok(v),
            Err(e) => Err(ErroHandle {
                level: retro_log_level::RETRO_LOG_ERROR,
                message: e.to_string(),
            }),
        }
    }
}
