use crc32fast::Hasher;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub fn crc32_file(path: impl AsRef<Path>) -> io::Result<u32> {
    let mut file = File::open(path)?;
    let mut hasher = Hasher::new();

    let mut buffer = [0u8; 64 * 1024]; // 64 KB

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    Ok(hasher.finalize())
}
