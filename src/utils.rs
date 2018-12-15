use std::io::BufReader;
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn read_input(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    BufReader::new(file).lines().into_iter().collect()
}
