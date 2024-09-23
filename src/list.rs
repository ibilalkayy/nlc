use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{Read, BufReader};
use walkdir::WalkDir;

pub async fn list_files(path: String) {
    let paths = WalkDir::new(path).into_iter().filter_map(|file| file.ok());

    for file in paths {
        if file.file_type().is_file() {
            
            let file_path = file.path();
            println!("{}", file.path().display());

            if let Ok(f) = File::open(file_path) {
                let mut reader = BufReader::new(f);
                let mut buffer = Vec::new();

                if let Ok(_) = reader.read_to_end(&mut buffer) {
                    let mut hash = Sha256::new();
                    hash.update(&buffer);
                    let hash_result = hash.finalize();

                    println!("Hash Result: {:x}", hash_result);
                } else {
                    println!("Could not read file content");
                }
            } else {
                println!("Could not open the file");
            }
        }
    }
}