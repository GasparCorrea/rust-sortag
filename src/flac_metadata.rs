use std::path::PathBuf;
use metaflac::Tag as Tag;

fn extract_key(tag: &Tag, key: &str) -> Result<String, ()> {
    let metadata = tag.get_vorbis(key);
    if metadata.is_some() {
        let metadata = metadata
        .unwrap()
        .next();
        if metadata.is_some() {
            return Ok(String::from(metadata.unwrap()));
        }
    }
    Err(())
}

pub fn extract_metadata(path: &PathBuf) -> Result<(String, String), &'static str> {
    let tag = Tag::read_from_path(path);
    if tag.is_err() {
        return Err("Error reading tag");
    }
    let tag = tag.unwrap();
    let artist = match extract_key(&tag, "ARTIST") {
        Ok(artist) => artist,
        _ => return Err("Error reading artist"),
    };
    let album = match extract_key(&tag, "ALBUM") {
        Ok(album) => album,
        _ => return Err("Error reading album"),
    };
    Ok((artist, album))
}
