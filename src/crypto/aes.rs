use aes_gcm::aead::{Aead, KeyInit, OsRng, generic_array::GenericArray};
use aes_gcm::{Aes256Gcm, Nonce}; // Or use Aes128Gcm
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const KEY_LEN: usize = 32;
const PBKDF2_ITER: u32 = 100_000;

pub fn derive_key(password: &str, salt: &[u8]) -> [u8; KEY_LEN] {
    let mut key = [0u8; KEY_LEN];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, PBKDF2_ITER, &mut key);
    key
}

pub fn encrypt(data: &[u8], password: &str) -> Result<Vec<u8>, String> {
    use rand::RngCore;

    let mut salt = [0u8; SALT_LEN];
    let mut nonce = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut salt);
    OsRng.fill_bytes(&mut nonce);

    let key = derive_key(password, &salt);
    let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));
    let nonce_arr = Nonce::from_slice(&nonce);

    let ciphertext = cipher
        .encrypt(nonce_arr, data)
        .map_err(|e| format!("Encryption failed: {}", e))?;

    // Output format: [salt | nonce | ciphertext]
    let mut out = Vec::with_capacity(SALT_LEN + NONCE_LEN + ciphertext.len());
    out.extend_from_slice(&salt);
    out.extend_from_slice(&nonce);
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

pub fn decrypt(data: &[u8], password: &str) -> Result<Vec<u8>, String> {
    if data.len() < SALT_LEN + NONCE_LEN {
        return Err("Input data too short".into());
    }
    let salt = &data[..SALT_LEN];
    let nonce = &data[SALT_LEN..SALT_LEN + NONCE_LEN];
    let ciphertext = &data[SALT_LEN + NONCE_LEN..];

    let key = derive_key(password, salt);
    let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));
    let nonce_arr = Nonce::from_slice(nonce);

    cipher
        .decrypt(nonce_arr, ciphertext)
        .map_err(|e| format!("Decryption failed: {}", e))
}