use base64::{engine::general_purpose, Engine};
use std::env;

/// Applies ROT13 cipher to a string
fn apply_rot13(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            'A'..='Z' => (((c as u8 - b'A' + 13) % 26) + b'A') as char,
            'a'..='z' => (((c as u8 - b'a' + 13) % 26) + b'a') as char,
            _ => c,
        })
        .collect()
}

/// Decodes a base64-encoded string into a UTF-8 string
fn decode_base64(input: &str) -> Result<String, String> {
    general_purpose::STANDARD
        .decode(input)
        .map_err(|e| format!("Base64 decode error: {}", e))
        .and_then(|bytes| {
            String::from_utf8(bytes).map_err(|e| format!("UTF-8 error: {}", e))
        })
}

/// Decrypts a string that was encrypted with: plaintext -> ROT13 -> base64 -> ROT13
fn decrypt_scrambled_text(encrypted: &str) -> Result<String, String> {
    let step1 = apply_rot13(encrypted);
    let step2 = decode_base64(&step1)?;
    let result = apply_rot13(&step2);
    Ok(result)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <encrypted_text>", args[0]);
        std::process::exit(1);
    }

    let encrypted = &args[1];

    match decrypt_scrambled_text(encrypted) {
        Ok(plaintext) => println!("Decrypted text: {}", plaintext),
        Err(err) => eprintln!("Error: {}", err),
    }
}