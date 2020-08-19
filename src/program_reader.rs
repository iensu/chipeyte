use std::fs;
use std::path::Path;

pub fn read(path: &Path) -> Vec<u8> {
    if !path.exists() {
        panic!("File {:?} does not exist!", path);
    }

    fs::read(path).expect("Something went wrong when reading the file!")
}
