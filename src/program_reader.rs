use std::fs;
use std::path::Path;

pub fn read(path: &Path) -> Vec<u16> {
    if !path.exists() {
        panic!("File {:?} does not exist!", path);
    }

    let contents = fs::read_to_string(path).expect("Something went wrong when reading the file!");

    contents
        .lines()
        .filter(|line| !line.starts_with(";"))
        .map(|line| u16::from_str_radix(line, 16).unwrap())
        .collect()
}
