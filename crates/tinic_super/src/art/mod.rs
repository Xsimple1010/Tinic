use crate::event::TinicSuperEventListener;
use crate::tools::download::download_file;
use generics::constants::THUMBNAIL_BASE_URL;
use generics::error_handle::ErrorHandle;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::sync::Arc;

pub enum ThumbnailType {
    Box,
    Snap,
    Titles,
}

impl Display for ThumbnailType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ThumbnailType::Box => "Named_Boxarts",
            ThumbnailType::Snap => "Named_Snaps",
            ThumbnailType::Titles => "Named_Titles",
        };
        write!(f, "{s}")
    }
}

pub fn get_thumbnail_url(thumbnail_type: ThumbnailType, sys_name: &str, name: &str) -> String {
    let name = name.replace(" ", "%20");
    let sys = sys_name.replace(" ", "%20");
    format!("{THUMBNAIL_BASE_URL}/{sys}/{thumbnail_type}/{name}.png")
}

pub async fn download_thumbnail(
    url: &str,
    name: &str,
    dest: PathBuf,
    event_listener: Arc<dyn TinicSuperEventListener>,
) -> Result<(), ErrorHandle> {
    let file_name = format!("{name}.png");

    let _ = download_file(url, &file_name, dest, false, event_listener)
        .await
        .map_err(|e| ErrorHandle::new(&e.to_string()))?;

    Ok(())
}

pub async fn download_all_thumbnail_from_game(
    sys_name: &str,
    name: &str,
    dest: &str,
    on_progress: Arc<dyn TinicSuperEventListener>,
) {
    let box_art = get_thumbnail_url(ThumbnailType::Box, sys_name, name);
    let snap_art = get_thumbnail_url(ThumbnailType::Snap, sys_name, name);
    let title_art = get_thumbnail_url(ThumbnailType::Titles, sys_name, name);

    let arts = [
        (box_art, ThumbnailType::Box),
        (snap_art, ThumbnailType::Snap),
        (title_art, ThumbnailType::Titles),
    ];

    for (art_url, art_type) in arts {
        let on_progress = on_progress.clone();
        let name = name.to_string();
        let dest = PathBuf::from(dest).join(art_type.to_string());

        tokio::spawn(async move {
            let _ = download_thumbnail(&art_url, &name, dest, on_progress).await;
        })
        .await
        .expect("TODO: panic message");
    }
}
