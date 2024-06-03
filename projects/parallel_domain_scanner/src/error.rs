// Import the Error trait from the thiserror crate
use thiserror::Error;

// Define a custom error type named Error using the thiserror crate
#[derive(Error, Debug, Clone)]
pub enum Error {
    // Define a variant for command line usage errors
    #[error("Usage: parallel_domain_scanner <github.com>")]
    CliUsage,

    // Define a variant for Reqwest errors, with a custom error message
    #[error("Reqwest: {0}")]
    Reqwest(String),
}

// Implement the conversion from reqwest::Error to our custom Error type
impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        // Convert the reqwest::Error to a String and wrap it in the Error::Reqwest variant
        Error::Reqwest(err.to_string())
    }
}