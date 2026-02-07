use futures_util::StreamExt;
use reqwest::Error;
use std::{fs::File, io::Write, path::PathBuf};

pub enum FileProgress {
    Download(String, f32),
    Extract(String),
}

pub async fn download_file<CA, CP>(
    url: &str,
    file_name: &str,
    out_dir: &str,
    force_update: bool,
    on_progress: CP,
    on_downloaded: CA,
) -> Result<(), Error>
where
    CA: Fn(PathBuf),
    CP: Fn(FileProgress) + Copy, // porcentagem (0.0 .. 100.0)
{
    let mut dest = PathBuf::from(out_dir);
    dest.push(file_name);
    let need_update = !dest.exists();

    if !need_update && !force_update {
        on_progress(FileProgress::Download(file_name.to_string(), 100.0));
        on_downloaded(dest);
        return Ok(());
    }

    let response = reqwest::get(url).await?;

    let total_size = response.content_length().unwrap_or(0);

    let mut file = File::create(&dest).unwrap();

    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).unwrap();

        downloaded += chunk.len() as u64;

        if total_size > 0 {
            let progress = (downloaded as f32 / total_size as f32) * 100.0;
            on_progress(FileProgress::Download(
                file_name.to_string(),
                progress.min(100.0),
            ));
        }
    }

    on_downloaded(dest);

    Ok(())
}
