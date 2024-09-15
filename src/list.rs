use std::fs;
use walkdir::WalkDir;

pub async fn list_files(path: String) {
    let paths = WalkDir::new(path).into_iter().filter_map(|file| file.ok());

    for file in paths {
        println!("{}", file.path().display())
    }
}