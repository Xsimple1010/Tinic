use generics::constants::THUMBNAIL_BASE_URL;

pub enum ThumbnailType {
    Box,
    Snap,
    Titles,
}

pub fn get_thumbnail_url(thumbnail_type: ThumbnailType, sys_name: &str, name: &str) -> String {
    let thumbnail_type = match thumbnail_type {
        ThumbnailType::Box => "Named_Boxarts",
        ThumbnailType::Snap => "Named_Snaps ",
        ThumbnailType::Titles => "Named_Titles",
    };

    format!("{THUMBNAIL_BASE_URL}/{sys_name}/{thumbnail_type}/{name}.png")
}
