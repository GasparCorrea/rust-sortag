use std::path::{PathBuf, Path};
use walkdir::WalkDir;
use clap::Parser;
use std::fs;

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

fn scan_directory(source : &str, target: &str) {
    for path in WalkDir::new(source) {
        let path = path.unwrap().into_path();
        let extension = get_extension(&path);
        let filename = String::from(path.file_name().unwrap().to_str().unwrap());
        // Extract metadata from mp3, flac files, skipping other files
        let metadata = match extension {
            Ok("mp3") => id3_metadata::extract_metadata(&path),
            Ok("flac") => flac_metadata::extract_metadata(&path),
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

        let from = path.to_str().unwrap();
        let to = format!("{target}/{artist}/{album}/");
        // Create folder
        if fs::create_dir_all(&to).is_err() {
            println!("Failed to create directory");
            continue;
        }

        // Move file to new folder
        let to = format!("{to}{filename}");
        match fs::rename(&from, &to) {
            Err(msg) => {
                println!("{msg}");
                continue;
            }
            _ => continue
        }
    }
}
/// Sort audio files in folders based in their metadata
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Path of the folder to sort
   source: String,

   /// Path of the destination folder
   target: String,

   /// Show files as they are moved
   #[arg(short, long, default_value_t = false)]
   verbose: bool,
}

fn validate_dirs(source : &str, target: &str) {
    let source = Path::new(&source);
    let target = Path::new(&target);

    // Check that both source and target are directories
    if !source.is_dir() {
        panic!("Source is not a directory");
    }
    if !target.is_dir() {
        panic!("Target is not a directory");
    }

    // Check that target is not a subdirectory of source to avoid loop traversal
    if target.starts_with(&source) {
        panic!("Target can't be subdirectory of source");
    }
}

fn main() {
    // Get Args
    let args = Args::parse();
    let source = args.source; 
    let target = args.target; 
    validate_dirs(&source, &target);
    // /Volumes/M5/
    // /Users/gasparcorrea/test
    println!("Scanning folder");
    println!("{}", args.verbose);
    scan_directory(&source, &target);
}