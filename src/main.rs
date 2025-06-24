mod cli;
mod errors;
mod utils;
mod crypto;

use cli::prompt::interactive_password_prompt;
use crypto::file_io::{encrypt_file, decrypt_file};
use errors::EncryptionError;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <encrypt|decrypt> <input_file> <output_file>", args[0]);
        process::exit(1);
    }

    let operation = &args[1];
    let input_file = &args[2];
    let output_file = &args[3];

    let password = interactive_password_prompt("Enter password: ").unwrap_or_else(|err| {
        eprintln!("Failed to read password: {}", err);
        process::exit(1);
    });

    let result = match operation.as_str() {
        "encrypt" => encrypt_file(input_file, output_file, &password),
        "decrypt" => decrypt_file(input_file, output_file, &password),
        _ => {
            eprintln!("Invalid operation: {}", operation);
            process::exit(1);
        }
    };

    match result {
        Ok(()) => println!("Success!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}