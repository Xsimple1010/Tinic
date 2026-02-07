use std::{
    ffi::NulError,
    sync::{MutexGuard, PoisonError, RwLockReadGuard, RwLockWriteGuard},
};
#[derive(Debug)]
pub struct ErroHandle {
    pub message: String,
}

impl ErroHandle {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
        }
    }
}

impl<T> From<PoisonError<MutexGuard<'_, T>>> for ErroHandle {
    fn from(op: PoisonError<MutexGuard<'_, T>>) -> Self {
        ErroHandle {
            message: op.to_string() + "Erro ao acessar o Mutex",
        }
    }
}

impl<T> From<PoisonError<RwLockWriteGuard<'_, T>>> for ErroHandle {
    fn from(op: PoisonError<RwLockWriteGuard<'_, T>>) -> Self {
        ErroHandle {
            message: op.to_string() + "Erro ao acessar o RwLock em modo escrita",
        }
    }
}

impl<T> From<PoisonError<RwLockReadGuard<'_, T>>> for ErroHandle {
    fn from(op: PoisonError<RwLockReadGuard<'_, T>>) -> Self {
        ErroHandle {
            message: op.to_string() + "Erro ao acessar o RwLock em modo escrita",
        }
    }
}

impl From<std::io::Error> for ErroHandle {
    fn from(op: std::io::Error) -> Self {
        ErroHandle {
            message: op.to_string() + "Erro ao acessar o RwLock em modo escrita",
        }
    }
}

impl From<NulError> for ErroHandle {
    fn from(value: NulError) -> Self {
        ErroHandle {
            message: "Erro ao tentar criar um cString: ".to_string() + value.to_string().as_str(),
        }
    }
}
