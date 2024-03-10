use std::fs::File;
use std::io;
use std::io::prelude::*;
use zip::result::ZipResult;
use zip::ZipArchive;

pub fn read_file_as_string(path: &str) -> io::Result<String> {
    let mut file = File::open(&path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn read_file_from_zip(zip_path: &str, filename: &str) -> io::Result<String> {
    let mut archive = get_zip_as_archive(zip_path)?;
    read_file_from_archive(&mut archive, filename)
}

pub fn read_file_from_archive(archive: &mut ZipArchive<File>, filename: &str) -> io::Result<String> {
    let mut buffer = String::new();
    archive.by_name(&filename)?.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn get_zip_as_archive(zip_path: &str) -> ZipResult<ZipArchive<File>> {
    let zip_file = File::open(&zip_path)?;
    ZipArchive::new(zip_file)
}