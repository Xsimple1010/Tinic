use crate::event::TinicSuperEventListener;
use crate::FileProgress;
use sevenz_rust::Error;
use std::collections::HashMap;
use std::io::BufWriter;
use std::sync::Arc;
use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::PathBuf,
};
use zip::ZipArchive;

pub fn extract_zip_file(
    file_path: PathBuf,
    out_dir: String,
    event_listener: Arc<dyn TinicSuperEventListener>,
) -> zip::result::ZipResult<()> {
    let file = File::open(file_path)?;
    let mut archive = ZipArchive::new(file)?;

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

            event_listener.extract_file(file.name().to_string());
        }
    }

    Ok(())
}

pub enum SevenZipBeforeExtractionAction {
    Extract,
    Jump,
    Stop,
}

pub fn extract_7zip_file<CP>(src_path: PathBuf, dest: String, mut before_extraction: CP)
where
    CP: FnMut(FileProgress) -> SevenZipBeforeExtractionAction,
{
    let dest_path = PathBuf::from(dest);
    let mut used_names = HashMap::<String, usize>::new();

    let _e = sevenz_rust::decompress_file_with_extract_fn(
        src_path,
        dest_path.clone(),
        |entry, reader, _| {
            if entry.is_directory() {
                return Ok(true);
            }

            // pega apenas o nome do arquivo (achata)
            let base_name = PathBuf::from(entry.name())
                .file_name()
                .and_then(|n| n.to_str())
                .ok_or(Error::BadTerminatedSubStreamsInfo)?
                .to_string();

            // evita sobrescrever arquivos com mesmo nome
            let count = used_names.entry(base_name.clone()).or_insert(0);
            let final_name = if *count == 0 {
                base_name.clone()
            } else {
                format!("{}_{}", count, base_name)
            };
            *count += 1;

            let action = before_extraction(FileProgress::Extract(final_name.clone()));

            match action {
                SevenZipBeforeExtractionAction::Jump => {
                    println!("jumping: {}", final_name);
                    // DRENA o stream
                    std::io::copy(reader, &mut std::io::sink())?;
                    Ok(true)
                }
                SevenZipBeforeExtractionAction::Extract => {
                    println!("Extraction: {}", final_name);
                    let file_path = dest_path.join(final_name);
                    let file = File::create(&file_path)?;
                    let mut writer = BufWriter::new(file);
                    std::io::copy(reader, &mut writer)?;
                    Ok(true)
                }
                SevenZipBeforeExtractionAction::Stop => {
                    println!("top: {}", final_name);
                    Ok(false)
                }
            }
        },
    );

    println!("{_e:?}")
}
