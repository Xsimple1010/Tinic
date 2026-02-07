use std::{
    fmt::{Display, Formatter},
    path::PathBuf,
};

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

pub type ImgUrl = String;

#[derive(Default)]
pub struct Thumbnails {
    pub box_img: (Option<PathBuf>, ImgUrl),
    pub snap_img: (Option<PathBuf>, ImgUrl),
    pub title_img: (Option<PathBuf>, ImgUrl),
}
