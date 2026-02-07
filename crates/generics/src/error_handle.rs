use std::{
    ffi::NulError,
    sync::{MutexGuard, PoisonError, RwLockReadGuard, RwLockWriteGuard},
};
#[derive(Debug)]
pub struct ErrorHandle {
    pub message: String,
}

impl ErrorHandle {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
        }
    }
}

impl From<String> for ErrorHandle {
    fn from(value: String) -> Self {
        ErrorHandle::new(&value)
    }
}

impl<T> From<PoisonError<MutexGuard<'_, T>>> for ErrorHandle {
    fn from(op: PoisonError<MutexGuard<'_, T>>) -> Self {
        ErrorHandle {
            message: op.to_string() + "Erro ao acessar o Mutex",
        }
    }
}

impl<T> From<PoisonError<RwLockWriteGuard<'_, T>>> for ErrorHandle {
    fn from(op: PoisonError<RwLockWriteGuard<'_, T>>) -> Self {
        ErrorHandle {
            message: op.to_string() + "Erro ao acessar o RwLock em modo escrita",
        }
    }
}

impl<T> From<PoisonError<RwLockReadGuard<'_, T>>> for ErrorHandle {
    fn from(op: PoisonError<RwLockReadGuard<'_, T>>) -> Self {
        ErrorHandle {
            message: op.to_string() + "Erro ao acessar o RwLock em modo escrita",
        }
    }
}

impl From<std::io::Error> for ErrorHandle {
    fn from(op: std::io::Error) -> Self {
        ErrorHandle {
            message: op.to_string() + "Erro ao acessar o RwLock em modo escrita",
        }
    }
}

impl From<NulError> for ErrorHandle {
    fn from(value: NulError) -> Self {
        ErrorHandle {
            message: "Erro ao tentar criar um cString: ".to_string() + value.to_string().as_str(),
        }
    }
}
