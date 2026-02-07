use generics::error_handle::ErrorHandle;
use generics::types::{ArcTMutex, TMutex};
use sqlite::Connection;
use std::path::PathBuf;
use std::sync::MutexGuard;

#[derive(Clone)]
pub struct TinicDbConnection {
    conn: ArcTMutex<Connection>,
}

impl TinicDbConnection {
    pub fn new(path: PathBuf) -> Result<Self, ErrorHandle> {
        let connection = sqlite::open(path.join("games.sqlite"))?;

        Ok(Self {
            conn: TMutex::new(connection),
        })
    }

    pub fn in_memory() -> Result<Self, ErrorHandle> {
        let connection = sqlite::open(":memory:")?;

        Ok(Self {
            conn: TMutex::new(connection),
        })
    }

    pub fn try_execute<T: AsRef<str>>(&self, statement: T) -> Result<(), ErrorHandle> {
        self.conn.try_load()?.execute(statement)?;
        Ok(())
    }

    pub fn execute<T: AsRef<str>>(&self, statement: T) -> Result<(), ErrorHandle> {
        Ok(self
            .conn
            .load_or_spawn_err("Não foi possivel liberar o mutex do sqlite::connection")?
            .execute(statement)?)
    }

    pub fn with_statement<F, R>(&self, query: &str, mut callback: F) -> Result<R, ErrorHandle>
    where
        F: FnMut(&mut sqlite::Statement, &MutexGuard<Connection>) -> Result<R, ErrorHandle>,
    {
        let conn = self
            .conn
            .load_or_spawn_err("Não foi possivel liberar o mutex do sqlite::connection")?;

        let mut stmt = conn.prepare(query)?;

        callback(&mut stmt, &conn)
    }
}
