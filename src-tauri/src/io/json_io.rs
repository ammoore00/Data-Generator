use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn read_file_as_string(path: &str) -> io::Result<String> {
    let mut file = File::open(&path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}