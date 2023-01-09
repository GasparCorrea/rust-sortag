use std::path::PathBuf;
use id3::{Tag, TagLike};

pub fn extract_metadata(path: &PathBuf) -> Result<(String, String), &'static str> {
    let tag = Tag::read_from_path(path);
    if tag.is_err() {
        return Err("Error reading tag");
    }
    let tag = tag.unwrap();
    let artist = tag.album_artist();
    let album = tag.album();
    let artist =  match artist {
        Some(artist) => String::from(artist),
        _ => return Err("Error reading artist")
    };
    let album =  match album {
        Some(album) => String::from(album),
        _ => return Err("Error reading album"),
    };
    Ok((artist, album))
}
