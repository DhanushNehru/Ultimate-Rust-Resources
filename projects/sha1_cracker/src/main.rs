use sha1::Digest;
// std::env imports the module env and others from the standard library
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

// Constant for the expected length of a SHA-1 hash string in hexadecimal format
const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    // A Vector is an array type that can be resized.
    // Collect the command-line arguments into a vector of strings.
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments is provided
    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
        return Ok(());
    }

    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is not valid".into());
    }

    // Open the wordlist file
    let wordlist_file = File::open(&args[1])?;
    // Create a buffered reader for the wordlist file to efficiently read lines
    let reader = BufReader::new(&wordlist_file);

    // Iterate over each line in the wordlist file
    for line in reader.lines() {
        let line = line?;
        let common_password = line.trim();

        // Calculate the SHA-1 hash of the current word and compare it to the target hash
        if hash_to_crack == &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
            println!("Password found: {}", &common_password);
            return Ok(());
        }
    }

    println!("password not found in wordlist :(");
    // as almost everything is an expression, this is equivalent to return Ok(());
    Ok(())
}