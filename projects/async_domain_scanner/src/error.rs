// Importing the `thiserror` crate for deriving error types
use thiserror::Error;

// Define a custom error type using the `Error` trait from `thiserror`
#[derive(Error, Debug, Clone)]
pub enum Error {
    // Error variant for incorrect command-line usage with a custom error message
    #[error("Usage: parallel_domain_scanner <github.com>")]
    CliUsage,
    // Error variant for handling errors from the `reqwest` library, converting the error message to a string
    #[error("Reqwest: {0}")]
    Reqwest(String),
}

// Implement conversion from `reqwest::Error` to our custom `Error` type
impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        // Convert the `reqwest::Error` into a `Reqwest` variant of our custom `Error` type
        Error::Reqwest(err.to_string())
    }
}

// Explanation of the modules used:
// thiserror: This crate simplifies the creation of error types. It provides a way to define custom error types
//            with less boilerplate code. Using `thiserror::Error`, we can derive the standard `Error` trait
//            for our custom error types, making them more ergonomic and easier to use.

// reqwest: This crate is used for making HTTP requests. Errors from this library need to be handled gracefully.
//          By implementing a conversion from `reqwest::Error` to our custom `Error` type, we ensure that our
//          application can handle and report errors from HTTP operations consistently and informatively.