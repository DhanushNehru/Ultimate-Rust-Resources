// Importing necessary crates and modules
use futures::{stream, StreamExt}; // For handling asynchronous streams
use reqwest::Client; // For making HTTP requests
use std::{
    env, // For accessing environment variables
    time::{Duration, Instant}, // For handling time durations and measuring elapsed time
};

// Importing custom error handling module
mod error;
pub use error::Error;

// Importing modules for handling subdomains and ports
mod model;
mod ports;
mod subdomains;
use model::Subdomain; // Importing the Subdomain model
mod common_ports; // Importing common_ports module for predefined ports

// Main function using Tokio's async runtime
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Collect command-line arguments into a vector of strings
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments is provided
    if args.len() != 2 {
        return Err(Error::CliUsage.into()); // Return an error if usage is incorrect
    }

    // Get the target from command-line arguments
    let target = args[1].as_str();

    // Set a timeout duration for HTTP requests
    let http_timeout = Duration::from_secs(10);
    // Build an HTTP client with a timeout
    let http_client = Client::builder().timeout(http_timeout).build()?;

    // Set concurrency limits for port scanning and subdomain enumeration
    let ports_concurrency = 200;
    let subdomains_concurrency = 100;
    // Record the start time for measuring scan duration
    let scan_start = Instant::now();

    // Enumerate subdomains using the HTTP client and target
    let subdomains = subdomains::enumerate(&http_client, target).await?;

    // Concurrent stream method 1: Using buffer_unordered + collect
    let scan_result: Vec<Subdomain> = stream::iter(subdomains.into_iter())
        .map(|subdomain| ports::scan_ports(ports_concurrency, subdomain))
        .buffer_unordered(subdomains_concurrency)
        .collect()
        .await;

    // Uncommented concurrent stream method 2: Using an Arc<Mutex<T>>
    // This method uses a shared, mutable state protected by a mutex
    // let res: Arc<Mutex<Vec<Subdomain>>> = Arc::new(Mutex::new(Vec::new()));

    // stream::iter(subdomains.into_iter())
    //     .for_each_concurrent(subdomains_concurrency, |subdomain| {
    //         let res = res.clone();
    //         async move {
    //             let subdomain = ports::scan_ports(ports_concurrency, subdomain).await;
    //             res.lock().await.push(subdomain)
    //         }
    //     })
    //     .await;

    // Measure the elapsed time for the scan
    let scan_duration = scan_start.elapsed();
    println!("Scan completed in {:?}", scan_duration);

    // Print the scan results: each subdomain and its open ports
    for subdomain in scan_result {
        println!("{}:", &subdomain.domain);
        for port in &subdomain.open_ports {
            println!("    {}: open", port.port);
        }

        println!("");
    }

    // Return Ok if the program completes successfully
    Ok(())
}

// Each module used:
// futures: Handles asynchronous streams, enabling efficient, concurrent processing.
// reqwest: Simplifies making HTTP requests, used here for fetching subdomain information.
// env: Accesses environment variables, such as command-line arguments.
// time: Measures durations and elapsed time for monitoring performance.
// error: Custom error handling for more descriptive error messages.
// model: Defines the data structures (e.g., Subdomain) used in the program.
// ports: Contains functions for port scanning, checking which ports are open on subdomains.
// subdomains: Contains functions for enumerating subdomains of a target domain.
// common_ports: Contains a list of common ports that are likely to be open, used for scanning.
