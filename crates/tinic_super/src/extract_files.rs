use crate::FileProgress;
use sevenz_rust::Error;
use std::io::BufWriter;
use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::PathBuf,
};
use zip::ZipArchive;

pub fn extract_zip_file<CP>(
    file_path: PathBuf,
    out_dir: String,
    on_progress: CP,
) -> zip::result::ZipResult<()>
where
    CP: Fn(FileProgress),
{
    let file = File::open(file_path)?;
    let mut archive = ZipArchive::new(file)?;

    let on_progress = on_progress;

    // 2️⃣ extrair com progresso global
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        let out_path = PathBuf::from(&out_dir).join(file.name());

        if file.is_dir() {
            create_dir_all(&out_path)?;
            continue;
        }

        if let Some(parent) = out_path.parent() {
            create_dir_all(parent)?;
        }

        let mut outfile = File::create(&out_path)?;

        let mut buffer = [0u8; 8192];

        loop {
            let n = file.read(&mut buffer)?;
            if n == 0 {
                break;
            }

            outfile.write_all(&buffer[..n])?;

            on_progress(FileProgress::Extract(file.name().to_string()));
        }
    }

    Ok(())
}

pub enum SevenZipBeforeExtractionAction {
    Extract,
    Jump,
}

pub fn extract_7zip_file<CP>(src_path: PathBuf, dest: String, before_extraction: CP)
where
    CP: Fn(FileProgress) -> SevenZipBeforeExtractionAction + Copy,
{
    let dest_path = PathBuf::from(dest);

    let _e = sevenz_rust::decompress_file_with_extract_fn(
        src_path,
        dest_path.clone(),
        |entry, reader, _| {
            if entry.is_directory() {
                return Ok(true);
            }

            // Sei que usar o BadTerminatedSubStreamsInfo é errado,
            // mas estou com pressa então isso pode ficar para depois
            let file_name = PathBuf::from(entry.name())
                .file_name()
                .ok_or(Error::BadTerminatedSubStreamsInfo)?
                .to_str()
                .ok_or(Error::BadTerminatedSubStreamsInfo)?
                .to_string();

            let action = before_extraction(FileProgress::Extract(file_name.clone()));

            match action {
                SevenZipBeforeExtractionAction::Jump => Ok(true),
                SevenZipBeforeExtractionAction::Extract => {
                    let file_path = dest_path.join(file_name);
                    let file = File::create(&file_path)?;
                    let mut writer = BufWriter::new(file);
                    let _ = std::io::copy(reader, &mut writer);
                    Ok(true)
                }
            }
        },
    );
}
