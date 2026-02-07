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

    let name = name.replace(" ", "%20");
    let sys = sys_name.replace(" ", "%20");
    format!("{THUMBNAIL_BASE_URL}/{sys}/{thumbnail_type}/{name}.png")
}
