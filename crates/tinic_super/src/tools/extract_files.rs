use sevenz_rust::Error;
use std::collections::HashMap;
use std::io::BufWriter;
use std::{
    fs::{File, create_dir_all},
    io::{Read, Write},
    path::PathBuf,
};
use zip::ZipArchive;

#[derive(Debug)]
pub enum ExtractProgress {
    Extracting {
        origin_file: String,
        inner_file_name: String,
    },
    Finished,
}

pub fn extract_zip_file<C>(
    file_path: PathBuf,
    out_dir: String,
    event_listener: C,
) -> zip::result::ZipResult<()>
where
    C: Fn(ExtractProgress),
{
    let file = File::open(&file_path)?;
    let origin_file = match file_path.file_prefix() {
        Some(name) => match name.to_str() {
            Some(name) => name.to_string(),
            None => return Err(zip::result::ZipError::FileNotFound),
        },
        None => return Err(zip::result::ZipError::FileNotFound),
    };

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

            event_listener(ExtractProgress::Extracting {
                origin_file: origin_file.clone(),
                inner_file_name: file.name().to_string(),
            });
        }
    }

    event_listener(ExtractProgress::Finished);

    Ok(())
}

pub enum SevenZipBeforeExtractionAction {
    Extract,
    Jump,
    Stop,
}

pub fn extract_7zip_file<C, CP>(
    src_path: PathBuf,
    dest: String,
    mut before_extraction: CP,
    event_listener: C,
) where
    C: Fn(ExtractProgress),
    CP: FnMut(String) -> SevenZipBeforeExtractionAction,
{
    let dest_path = PathBuf::from(dest);
    let origin_file = match src_path.file_prefix() {
        Some(name) => match name.to_str() {
            Some(name) => name.to_string(),
            None => return,
        },
        None => return,
    };
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

            let action = before_extraction(final_name.clone());

            match action {
                SevenZipBeforeExtractionAction::Jump => {
                    // DRENA o stream
                    std::io::copy(reader, &mut std::io::sink())?;
                    Ok(true)
                }
                SevenZipBeforeExtractionAction::Extract => {
                    let file_path = dest_path.join(final_name.clone());
                    let file = File::create(&file_path)?;
                    let mut writer = BufWriter::new(file);
                    std::io::copy(reader, &mut writer)?;

                    event_listener(ExtractProgress::Extracting {
                        origin_file: origin_file.clone(),
                        inner_file_name: final_name.clone(),
                    });

                    Ok(true)
                }
                SevenZipBeforeExtractionAction::Stop => Ok(false),
            }
        },
    );

    event_listener(ExtractProgress::Finished);

    println!("{_e:?}")
}
