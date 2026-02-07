use crate::infos::model::CoreInfo;
use crate::infos::read_file::read_info_file;
use std::path::PathBuf;

pub async fn get_all_core_infos(dir: &String) -> Vec<CoreInfo> {
    let path = PathBuf::from(dir);

    let mut read_dir = tokio::fs::read_dir(path).await.unwrap();

    let mut infos = Vec::new();

    while let Ok(Some(entry)) = read_dir.next_entry().await {
        match read_info_file(&entry.path()).await {
            Ok(info) => infos.push(info),
            Err(_) => continue,
        };
    }

    infos
}
