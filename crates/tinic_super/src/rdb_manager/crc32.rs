use crc32fast::Hasher;
use std::io::{self};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn crc32_file(mut file: File) -> io::Result<u32> {
    let mut hasher = Hasher::new();

    let mut buffer = [0u8; 64 * 1024]; // 64 KB

    loop {
        let n = file.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    Ok(hasher.finalize())
}
