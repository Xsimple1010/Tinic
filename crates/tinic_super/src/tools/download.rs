use crate::event::TinicSuperEventListener;
use futures_util::StreamExt;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub enum DownloadProgress {
    Started(String),
    Progress(String, f32),
    Completed(String),
}

pub async fn download_file(
    url: &str,
    file_name: &str,
    mut dest: PathBuf,
    force_update: bool,
    event_listener: Arc<dyn TinicSuperEventListener>,
) -> Result<PathBuf, tokio::io::Error> {
    if !dest.exists() {
        fs::create_dir_all(&dest).await?;
    }

    let response = reqwest::get(url)
        .await
        .map_err(|e| tokio::io::Error::new(tokio::io::ErrorKind::Other, e))?;

    if response.status() != reqwest::StatusCode::OK {
        return Err(tokio::io::Error::new(
            tokio::io::ErrorKind::Other,
            "invalid status code",
        ));
    }

    dest.push(file_name);
    let need_update = !dest.exists();

    if !need_update && !force_update {
        event_listener.downloading(DownloadProgress::Completed(file_name.to_string()));
        return Ok(dest);
    }

    let mut file = File::create(&dest).await?;

    let mut downloaded: u64 = 0;
    let total_size = response.content_length().unwrap_or(0);
    let mut stream = response.bytes_stream();

    event_listener.downloading(DownloadProgress::Started(file_name.to_string()));

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| tokio::io::Error::new(tokio::io::ErrorKind::Other, e))?;
        file.write_all(&chunk).await?;

        downloaded += chunk.len() as u64;

        if total_size > 0 {
            let progress = (downloaded as f32 / total_size as f32) * 100.0;
            event_listener.downloading(DownloadProgress::Progress(
                file_name.to_string(),
                progress.min(100.0),
            ));
        }
    }

    event_listener.downloading(DownloadProgress::Completed(file_name.to_string()));

    Ok(dest)
}
