use crc32fast::Hasher;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub fn crc32_file(path: impl AsRef<Path>) -> io::Result<u32> {
    let mut file = File::open(path)?;
    let mut hasher = Hasher::new();

    let mut buffer = Vec::new(); // 64 KB
    let n = file.read_to_end(&mut buffer)?;
    hasher.update(&buffer[..n]);

    Ok(hasher.finalize())
}
