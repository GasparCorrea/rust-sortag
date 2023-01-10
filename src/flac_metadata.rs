use std::path::PathBuf;
use metaflac::Tag as Tag;

fn read_key(tag: &Tag, key: &str) -> Result<String, ()> {
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
    let tag = match Tag::read_from_path(path) {
        Ok(tag) => tag,
        _ => return  Err("Error reading tag"),
    };
    let artist = match read_key(&tag, "ARTIST") {
        Ok(artist) => artist,
        _ => return Err("Error reading artist"),
    };
    let album = match read_key(&tag, "ALBUM") {
        Ok(album) => album,
        _ => return Err("Error reading album"),
    };
    Ok((artist, album))
}
