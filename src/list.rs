use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{Error, Read, BufReader};
use walkdir::WalkDir;
use std::fs;

fn file_size(path: &str) -> Result<u64, std::io::Error> {
    let metadata = fs::metadata(path)?.len();
    Ok(metadata)
}

pub async fn list_files(path: String) {
    let paths = WalkDir::new(path).into_iter().filter_map(|file| file.ok());

    for file in paths {
        if file.file_type().is_file() {
            let file_path = file.path();
            
            if let Some(file_path_str) = file_path.to_str() {
                if let Ok(f) = File::open(file_path) {
                    let mut reader = BufReader::new(f);
                    let mut buffer = Vec::new();

                    if let Ok(_) = reader.read_to_end(&mut buffer) {
                        let mut hash = Sha256::new();
                        hash.update(&buffer);
                        let hash_result = hash.finalize();

                        match file_size(file_path_str) {
                            Ok(size) => println!("File: {}, Hash value: {:x} File size: {} bytes", file.path().display(), hash_result, size),
                            Err(err) => println!("Failed to get the file size: {}", err),
                        }
                    } else {
                        println!("Could not read file content");
                    }
                } else {
                    println!("Could not open the file");
                }
            } else {
                println!("Failed to convert file path to string");
            }
        }
    }
}