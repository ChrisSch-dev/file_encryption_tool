use crate::crypto::aes::{encrypt, decrypt};
use crate::errors::EncryptionError;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

const CHUNK_SIZE: usize = 1024 * 1024; // 1MB

pub fn encrypt_file(
    input_path: &str,
    output_path: &str,
    password: &str,
) -> Result<(), EncryptionError> {
    let input = File::open(input_path)?;
    let mut reader = BufReader::new(input);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let encrypted = encrypt(&buffer, password).map_err(EncryptionError::Crypto)?;
    let output = File::create(output_path)?;
    let mut writer = BufWriter::new(output);
    writer.write_all(&encrypted)?;
    Ok(())
}

pub fn decrypt_file(
    input_path: &str,
    output_path: &str,
    password: &str,
) -> Result<(), EncryptionError> {
    let input = File::open(input_path)?;
    let mut reader = BufReader::new(input);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let decrypted = decrypt(&buffer, password).map_err(EncryptionError::Crypto)?;
    let output = File::create(output_path)?;
    let mut writer = BufWriter::new(output);
    writer.write_all(&decrypted)?;
    Ok(())
}