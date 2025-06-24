use rpassword::read_password;
use std::io::{self, Write};

pub fn interactive_password_prompt(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?; // Ensure the prompt is printed before reading
    read_password()
}