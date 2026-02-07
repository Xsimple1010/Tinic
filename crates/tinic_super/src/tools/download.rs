use futures_util::StreamExt;
use std::path::PathBuf;
use tokio::fs;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, Error, ErrorKind};

#[derive(Debug)]
pub enum DownloadProgress {
    Started { name: String },
    Progress { name: String, progress: f32 },
    Completed { name: String },
}

pub async fn download_file<C>(
    url: &str,
    file_name: &str,
    mut dest: PathBuf,
    force_update: bool,
    event_listener: C,
) -> Result<PathBuf, Error>
where
    C: Fn(DownloadProgress),
{
    if !dest.exists() {
        fs::create_dir_all(&dest).await?;
    }

    let response = reqwest::get(url)
        .await
        .map_err(|e| Error::new(ErrorKind::Other, e))?;

    if response.status() != reqwest::StatusCode::OK {
        return Err(Error::new(ErrorKind::Other, "invalid status code"));
    }

    dest.push(file_name);
    let need_update = !dest.exists();

    if !need_update && !force_update {
        event_listener(DownloadProgress::Completed {
            name: file_name.to_string(),
        });
        return Ok(dest);
    }

    let mut file = File::create(&dest).await?;

    let mut downloaded: u64 = 0;
    let total_size = response.content_length().unwrap_or(0);
    let mut stream = response.bytes_stream();

    event_listener(DownloadProgress::Started {
        name: file_name.to_string(),
    });

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| Error::new(ErrorKind::Other, e))?;
        file.write_all(&chunk).await?;

        downloaded += chunk.len() as u64;

        if total_size > 0 {
            let progress = (downloaded as f32 / total_size as f32) * 100.0;
            event_listener(DownloadProgress::Progress {
                name: file_name.to_string(),
                progress,
            });
        }
    }

    event_listener(DownloadProgress::Completed {
        name: file_name.to_string(),
    });

    Ok(dest)
}
