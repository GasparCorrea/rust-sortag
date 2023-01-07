use std::path::{PathBuf, Path};
use walkdir::WalkDir;

// Metadata
mod id3_metadata;
mod flac_metadata;

fn get_extension(path: &PathBuf) -> Result<&'static str, ()> {
    let extension  = path.extension();
    if extension.is_none() {
        return Err(());
    }
    let extension = extension.unwrap().to_str();
    if extension.is_none() {
        return Err(());
    }
    let extension = match extension.unwrap() {
        "mp3" => Ok("mp3"),
        "flac" => Ok("flac"),
        _ => Err(())
    };
    extension
}

fn scan_directory(root : &str) {
    for path in WalkDir::new(root) {
        let path = path.unwrap().into_path();
        let extension = get_extension(&path);
        let filename = String::from(path.file_name().unwrap().to_str().unwrap());
        // Extract metadata from mp3, flac files, skipping other files
        let metadata = match extension {
            Ok("mp3") => id3_metadata::extract_metadata(path),
            Ok("flac") => flac_metadata::extract_metadata(path),
            _ => continue,
        };
        // Check if extracting the metadata was succesful
        let (artist, album) = match metadata {
            Ok(result) => result,
            Err(msg) => {
                println!("{filename}: {msg}");
                continue;
            }
        };
        let destination = format!("{root}/{artist}/{album}/{filename}");
        println!("{destination}");
    }
}

fn main() {
    println!("Scanning folder");
    // /Volumes/M5/
    // /Users/gasparcorrea/test
    if !Path::new("/Volumes/M5").is_dir() {
        panic!("Root is not a folder");
    }
    scan_directory("/Volumes/M5/");
}