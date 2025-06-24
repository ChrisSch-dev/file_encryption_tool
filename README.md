# File Encryption Tool

A modular, extensible file encryption tool written in Rust.  
**Features include:**

- AES-GCM and ChaCha20-Poly1305 encryption
- Interactive password prompt (with strength meter)
- Support for large files (with progress bar)
- File integrity verification (HMAC/SHA256)
- Compression before encryption
- Secure file shredding (wiping originals)
- Batch & wildcard file processing
- File metadata preservation (timestamps, permissions)
- Encrypted filenames
- Keyfile support (use key files for encryption)
- Configurable algorithms and parameters
- Config file support
- Log/audit trail support
- Optional GUI frontend (egui/eframe)
- Optional cloud integration (AWS S3, Dropbox)
- Extensive error handling
- Integration and property-based tests

---

## Usage

```
cargo run -- <encrypt|decrypt> <input_file> <output_file> [options]
```
Or, for the GUI:
```
cargo run --features gui
```

### Example

```
cargo run -- encrypt secret.txt secret.txt.enc
cargo run -- decrypt secret.txt.enc secret.txt
```

You will be prompted for a password (input is hidden and strength is shown).

---

## Configuration

You can use a TOML config file for default settings (see `config/settings.rs`).  
Example:
```toml
algorithm = "aes-gcm"
keyfile = "/path/to/keyfile"
compress = true
verify_integrity = true
cloud_upload = "s3"
```

---

## Features

- **Encryption Algorithms:** AES-256-GCM (default), ChaCha20-Poly1305 (optional)
- **Compression:** Optionally compress files before encryption
- **Progress Bar:** See progress for large files
- **Batch Processing:** Encrypt/decrypt multiple files at once
- **Cloud Integration:** Upload/download encrypted files from S3 or Dropbox
- **Secure Shredding:** Overwrite and remove originals
- **Password Strength Meter:** Get feedback on password strength
- **Keyfile Support:** Use a file as part of the key
- **GUI:** Optional desktop user interface

---

## Building

Requires Rust 1.70+.

```
cargo build --release
```

### Optional Features

- `gui` — Enables GUI frontend
- `cloud` — Enables S3 and Dropbox integration
- `compression` — Enables compression support
- `extra-ciphers` — Enables ChaCha20-Poly1305

Enable with:
```
cargo build --release --features "gui cloud compression extra-ciphers"
```

---

## Testing

```
cargo test
```

---

## Security Notes

- Passwords are never stored or logged.
- Uses PBKDF2 (100,000 iterations) for key derivation by default.
- Salt and nonce are randomly generated per file.
- HMAC/SHA256 verifies file integrity.
- Shredding overwrites files before deletion (best effort).

---

## License

MIT

---

## Contributions

Contributions are welcome!  
Please open issues or pull requests.

---

## Authors

- [ChrisSch-dev](https://github.com/ChrisSch-dev)
