use std::fs;

pub fn read_keyfile(path: &str) -> std::io::Result<Vec<u8>> {
    fs::read(path)
}