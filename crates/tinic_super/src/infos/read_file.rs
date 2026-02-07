use crate::infos::model::CoreInfo;
use generics::error_handle::ErrorHandle;
use std::{ffi::OsStr, path::PathBuf};

fn create_and_join_core_name(file_name: Option<&OsStr>) -> Result<String, ErrorHandle> {
    let file_name = match file_name {
        Some(file_name) => file_name.to_str().ok_or(ErrorHandle {
            message: "Nome do arquivo inválido".to_string(),
        })?,
        None => {
            return Err(ErrorHandle {
                message: "Nome do arquivo não fornecido".to_string(),
            });
        }
    };

    if cfg!(target_os = "windows") {
        Ok(format!("{}.dll", file_name))
    } else if cfg!(target_os = "linux") {
        Ok(format!("{}.so", file_name))
    } else {
        return Err(ErrorHandle {
            message: "Sistema operacional não suportado".to_string(),
        });
    }
}

pub async fn read_info_file(
    file_path: &PathBuf,
    core_dir: &PathBuf,
) -> Result<CoreInfo, ErrorHandle> {
    use tokio::fs::File;
    use tokio::io::{AsyncBufReadExt, BufReader};

    let file = File::open(file_path).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut info = CoreInfo::default();

    info.info_path = file_path.clone();

    let core_name = create_and_join_core_name(file_path.file_prefix())?;
    info.core_path = core_dir.join(core_name);

    while let Ok(Some(mut line)) = lines.next_line().await {
        set_info_value(&mut line, &mut info);
    }

    set_file_name_info(file_path, &mut info)?;

    Ok(info)
}

pub fn read_info_file_blocking(
    file_path: &PathBuf,
    core_dir: &PathBuf,
) -> Result<CoreInfo, ErrorHandle> {
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut info = CoreInfo::default();
    info.info_path = file_path.clone();

    let core_name = create_and_join_core_name(file_path.file_prefix())?;
    info.core_path = core_dir.join(core_name);

    while let Some(Ok(mut line)) = lines.next() {
        set_info_value(&mut line, &mut info);
    }

    set_file_name_info(file_path, &mut info)?;

    Ok(info)
}

fn set_info_value(line: &mut str, info: &mut CoreInfo) {
    if let Some((key, value)) = line.split_once('=') {
        info.set_value(
            key.trim(),
            value
                .trim_matches('"')
                .replacen(" ", "", 1)
                .replacen('\"', "", 1)
                .to_string(),
        );
    }
}

fn set_file_name_info(file_path: &PathBuf, info: &mut CoreInfo) -> Result<(), ErrorHandle> {
    info.file_name = file_path
        .file_name()
        .ok_or(ErrorHandle::new("File has no file name"))?
        .to_str()
        .ok_or(ErrorHandle::new("File has no file name"))?
        .to_string()
        .replace(".info", "");

    Ok(())
}
