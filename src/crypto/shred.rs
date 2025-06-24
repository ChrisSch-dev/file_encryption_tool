use std::fs::{OpenOptions, remove_file};
use std::io::{Write, Seek, SeekFrom};

pub fn shred_file(path: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true).open(path)?;
    let len = file.metadata()?.len();
    file.seek(SeekFrom::Start(0))?;
    file.write_all(&vec![0u8; len as usize])?;
    file.sync_all()?;
    drop(file);
    remove_file(path)
}