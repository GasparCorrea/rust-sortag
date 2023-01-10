use std::path::PathBuf;
use id3::{Tag, TagLike};

pub fn extract_metadata(path: &PathBuf) -> Result<(String, String), &'static str> {
    let tag = match Tag::read_from_path(path) {
        Ok(tag) => tag,
        _ => return Err("Error reading tag"),
    };
    let artist = match  tag.album_artist() {
        Some(artist) => String::from(artist),
        _ => return Err("Error reading artist")
    };
    let album = match tag.album(){ 
        Some(album) => String::from(album),
        _ => return Err("Error reading album"),
    };
    Ok((artist, album))
}
