use crate::rdb_manager::crc32::crc32_file;
use generics::error_handle::ErrorHandle;
use std::path::PathBuf;
use tokio::fs::{File, read_dir};

#[derive(Debug)]
pub struct GameIdentifier {
    pub path: PathBuf,
    pub file_name: String,
    pub crc: u32,
    pub size: u64,
}

const BLACKLIST_EXTENSIONS: &[&str] = &[
    "txt", "nfo", "jpg", "jpeg", "png", "gif", "xml", "json", "ini", "cfg", "md", "db", "sqlite",
    "log", "zip", "7z",
];

impl GameIdentifier {
    pub async fn new(path: PathBuf) -> Result<Self, ErrorHandle> {
        if !Self::is_probably_rom(&path) {
            return Err(ErrorHandle::new("arquivo invalido"));
        }

        let file = File::open(path.clone()).await?;
        let size = file.metadata().await?.len();
        let crc = crc32_file(file).await?;
        let rom_name = match path.file_prefix() {
            Some(name) => match name.to_str() {
                Some(name) => name.to_string(),
                None => return Err(ErrorHandle::new("Não foi possivel recuperar o nome da rom")),
            },
            None => return Err(ErrorHandle::new("Não foi possivel recuperar o nome da rom")),
        };

        Ok(Self {
            crc,
            path,
            size,
            file_name: rom_name,
        })
    }

    fn is_probably_rom(path: &PathBuf) -> bool {
        let ext = match path.extension().and_then(|e| e.to_str()) {
            Some(ext) => ext.to_lowercase(),
            None => return false,
        };

        if BLACKLIST_EXTENSIONS.contains(&ext.as_str()) {
            return false;
        }

        true
    }

    pub async fn from_dir(dir: PathBuf) -> Result<Vec<Self>, ErrorHandle> {
        let mut read_dir = read_dir(dir).await?;
        let mut out = Vec::new();

        while let Some(dir_entry) = read_dir.next_entry().await? {
            if dir_entry.metadata().await?.is_dir() {
                continue;
            }

            let res = GameIdentifier::new(dir_entry.path()).await;

            if let Ok(ident) = res {
                out.push(ident);
            }
        }

        Ok(out)
    }
}
