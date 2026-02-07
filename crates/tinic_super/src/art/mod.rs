use crate::art::helper::ThumbnailEventType;
use crate::art::thumbnail::{ThumbnailType, Thumbnails};
use crate::event::TinicSuperEventListener;
use crate::tools::download::download_file;
use generics::constants::THUMBNAIL_BASE_URL;
use generics::error_handle::ErrorHandle;
use std::fmt::Display;
use std::path::PathBuf;
use std::sync::Arc;

pub mod helper;
pub mod thumbnail;

pub fn get_thumbnail_url(
    thumbnail_type: &ThumbnailType,
    rdb_name: &impl Display,
    name: &impl Display,
) -> String {
    let name = name.to_string().replace(" ", "%20");
    let sys = rdb_name.to_string().replace(" ", "%20");
    format!("{THUMBNAIL_BASE_URL}/{sys}/{thumbnail_type}/{name}.png")
}

pub async fn download_thumbnail(
    url: &str,
    name: &str,
    dest: PathBuf,
    event_listener: Arc<dyn TinicSuperEventListener>,
) -> Result<PathBuf, ErrorHandle> {
    let file_name = format!("{name}.png");

    let path = download_file(url, &file_name, dest, false, |event| {
        event_listener.on_thumbnail_evnt(ThumbnailEventType::Downloading(event));
    })
    .await
    .map_err(|e| ErrorHandle::new(&e.to_string()))?;

    Ok(path)
}

pub async fn download_all_thumbnail_from_game(
    rdb_name: &impl Display,
    name: &impl Display,
    dest: &String,
    on_progress: Arc<dyn TinicSuperEventListener>,
) -> Result<Thumbnails, ErrorHandle> {
    let box_art = get_thumbnail_url(&ThumbnailType::Box, rdb_name, name);
    let snap_art = get_thumbnail_url(&ThumbnailType::Snap, rdb_name, name);
    let title_art = get_thumbnail_url(&ThumbnailType::Titles, rdb_name, name);

    let arts = [
        (box_art, ThumbnailType::Box),
        (snap_art, ThumbnailType::Snap),
        (title_art, ThumbnailType::Titles),
    ];

    let mut thumbnails = Thumbnails::default();

    for (art_url, art_type) in arts {
        let on_progress = on_progress.clone();
        let name = name.to_string();
        let dest = PathBuf::from(dest).join(art_type.to_string());
        let art_url_2 = art_url.clone();

        let path = tokio::spawn(async move {
            download_thumbnail(&art_url_2, &name, dest, on_progress)
                .await
                .ok()
        })
        .await
        .ok()
        .flatten();

        match art_type {
            ThumbnailType::Box => thumbnails.box_img = (path, art_url),
            ThumbnailType::Snap => thumbnails.snap_img = (path, art_url),
            ThumbnailType::Titles => thumbnails.title_img = (path, art_url),
        }
    }

    Ok(thumbnails)
}
