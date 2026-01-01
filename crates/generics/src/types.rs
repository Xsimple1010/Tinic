use crate::error_handle::ErrorHandle;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

pub type ArcTMutex<T> = Arc<TMutex<T>>;

#[derive(Debug)]
pub struct TMutex<T> {
    value: Mutex<T>,
}

impl<T> TMutex<T> {
    pub fn new(value: T) -> ArcTMutex<T> {
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

    pub fn load_or_spaw_err(&self, error_menssage: &str) -> Result<MutexGuard<'_, T>, ErrorHandle> {
        match self.value.lock() {
            Ok(v) => Ok(v),
            Err(_) => Err(ErrorHandle::new(error_menssage)),
        }
    }

    pub fn store_or_else<CA: FnOnce(PoisonError<MutexGuard<'_, T>>)>(&self, value: T, or_else: CA) {
        match self.value.lock() {
            Ok(mut v) => *v = value,
            Err(e) => or_else(e),
        }
    }

    pub fn try_load(&self) -> Result<MutexGuard<'_, T>, ErrorHandle> {
        match self.value.try_lock() {
            Ok(v) => Ok(v),
            Err(e) => Err(ErrorHandle {
                message: e.to_string(),
            }),
        }
    }
}
