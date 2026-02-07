use std::{fs::File, io::copy, path::PathBuf};

use reqwest::Error;

pub async fn download_file<CA>(
    url: &str,
    file_name: &str,
    out_dir: &str,
    force_update: bool,
    on_downloaded: CA,
) -> Result<(), Error>
where
    CA: Fn(PathBuf),
{
    let mut dest = PathBuf::from(out_dir);
    let need_update = !dest.exists();

    if need_update || force_update {
        let response = reqwest::get(url).await?;

        let bytes = response.bytes().await?;

        dest.push(file_name);

        let mut file = File::create(&dest).unwrap();
        copy(&mut bytes.as_ref(), &mut file).unwrap();

        on_downloaded(dest);

        return Ok(());
    }

    Ok(())
}
