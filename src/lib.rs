use std::fs::File;
use std::io::*;

pub fn read_input(filename: &str) -> Vec<String> {
    let file = File::open(filename);
    let reader = BufReader::new(file.unwrap());

    return reader.lines().map(|x| x.unwrap()).collect();
}
