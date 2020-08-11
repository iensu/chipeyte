use std::fs;
use std::{collections::HashMap, path::Path};

pub fn read(path: &Path) -> HashMap<usize, u16> {
    if !path.exists() {
        panic!("File {:?} does not exist!", path);
    }

    let contents = fs::read_to_string(path).expect("Something went wrong when reading the file!");

    contents
        .lines()
        .filter(|line| !line.starts_with(";") && line.trim().len() > 0)
        .fold(HashMap::<usize, u16>::new(), |mut acc, line| {
            let items = line.split(":").collect::<Vec<&str>>();
            let address = usize::from_str_radix(items[0].trim(), 16).unwrap();
            let instruction = u16::from_str_radix(items[1].trim(), 16).unwrap();

            acc.insert(address, instruction);
            acc
        })
}
