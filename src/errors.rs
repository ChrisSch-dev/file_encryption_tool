use std::fmt;
use std::io;

#[derive(Debug)]
pub enum EncryptionError {
    Io(io::Error),
    Crypto(String),
}

impl fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncryptionError::Io(e) => write!(f, "IO error: {}", e),
            EncryptionError::Crypto(e) => write!(f, "Crypto error: {}", e),
        }
    }
}

impl std::error::Error for EncryptionError {}

impl From<io::Error> for EncryptionError {
    fn from(err: io::Error) -> EncryptionError {
        EncryptionError::Io(err)
    }
}