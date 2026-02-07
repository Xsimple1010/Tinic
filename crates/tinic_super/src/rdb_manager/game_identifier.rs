use crate::rdb_manager::crc32::crc32_file;
use generics::error_handle::ErrorHandle;
use std::path::PathBuf;
use tokio::fs::File;

pub struct GameIdentifier {
    pub path: PathBuf,
    pub rom_name: String,
    pub crc: u32,
    pub size: u64,
}

impl GameIdentifier {
    pub async fn new(path: PathBuf) -> Result<Self, ErrorHandle> {
        let file = File::open(path.clone()).await?;
        let size = file.metadata().await?.len();
        let crc = crc32_file(file).await?;
        let rom_name = match path.file_name() {
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
            rom_name,
        })
    }
}
