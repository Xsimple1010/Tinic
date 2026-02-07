use sqlite::Value;

#[derive(Debug, Default)]
pub struct GameInfoInDb {
    pub name: Option<String>,
    pub description: Option<String>,
    pub genre: Option<String>,
    pub developer: Option<String>,
    pub publisher: Option<String>,
    pub franchise: Option<String>,
    pub origin: Option<String>,
    pub rom_name: Option<String>,
    pub release_year: Option<u32>,
    pub release_month: Option<u32>,
    pub size: Option<u64>,
    pub crc32: Option<u32>,
    pub serial: Option<String>,
    pub core_path: Option<String>,
    pub rom_path: Option<String>,
    pub rumble: bool,
    pub console_name: Option<String>,
}

#[derive(Debug)]
pub struct GameInfoPagination {
    pub id: i64,
    pub name: Option<String>,
    pub rom_path: String,
    pub console_name: Option<String>,
}


pub(crate) fn opt_str(v: &Option<String>) -> Value {
    match v {
        Some(s) => Value::String(s.clone()),
        None => Value::Null,
    }
}

pub(crate) fn opt_u64(v: Option<u64>) -> Value {
    match v {
        Some(n) => Value::Integer(n as i64),
        None => Value::Null,
    }
}

pub(crate) fn opt_u32(v: Option<u32>) -> Value {
    match v {
        Some(n) => Value::Integer(n as i64),
        None => Value::Null,
    }
}

pub(crate) fn opt_bool(v: bool) -> Value {
    Value::Integer(if v { 1 } else { 0 })
}
