// rayon, a data-parallelism library for Rust
use rayon::prelude::*; // Imports Rayon parallel iterator traits for parallel processing
use reqwest::{blocking::Client, redirect}; // Imports Reqwest for making HTTP requests
use std::{env, time::Duration}; // Imports standard library modules for environment variables and time duration

// Importing custom error handling module
mod error;
pub use error::Error;

// Importing model, ports, and subdomains modules
mod model;
mod ports;
mod subdomains;
use model::Subdomain; // Importing the Subdomain model from the model module
mod common_ports; // Importing common_ports module

fn main() -> Result<(), anyhow::Error> {
    // Collect command-line arguments into a vector of strings
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments is provided
    if args.len() != 2 {
        return Err(Error::CliUsage.into()); // Return an error if usage is incorrect
    }

    // Get the target from command-line arguments
    let target = args[1].as_str();

    // Set a timeout duration for HTTP requests
    let http_timeout = Duration::from_secs(5);
    // Build an HTTP client with a redirect policy and timeout
    let http_client = Client::builder()
        .redirect(redirect::Policy::limited(4))
        .timeout(http_timeout)
        .build()?;

    // Create a custom thread pool with 256 threads to improve speed
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(256)
        .build()
        .unwrap();

    // Use the custom thread pool to execute the following block of code
    // The `||` { syntax defines an anonymous closure. In Rust, a closure is a function-like construct that can capture variables from its surrounding environment
    // `||` signifies that the closure takes no arguments.    
    pool.install(|| {
        // Enumerate subdomains using the HTTP client and target, then scan ports in parallel
        let scan_result: Vec<Subdomain> = subdomains::enumerate(&http_client, target)
            .unwrap() // Unwrap the result to handle potential errors
            .into_par_iter() // Convert the result into a parallel iterator
            .map(ports::scan_ports) // Map each subdomain to scan_ports function
            .collect(); // Collect the results into a vector

        // Iterate over the scan results and print each subdomain and its open ports
        for subdomain in scan_result {
            println!("{}:", &subdomain.domain);
            for port in &subdomain.open_ports {
                println!("    {}", port.port);
            }

            println!();
        }
    });

    Ok(()) // Return Ok if the program completes successfully
}