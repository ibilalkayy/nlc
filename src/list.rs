use std::fs;

pub fn list_files(path: String) {
    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}