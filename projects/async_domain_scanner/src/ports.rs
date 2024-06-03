use crate::{
    common_ports::MOST_COMMON_PORTS_100, // Importing the list of the 100 most common ports from the common_ports module
    model::{Port, Subdomain}, // Importing the Port and Subdomain structs from the model module
};
use futures::StreamExt; // Importing StreamExt for additional stream methods
use std::net::{SocketAddr, ToSocketAddrs}; // Importing standard library modules for handling socket addresses
use std::time::Duration; // Importing Duration for time handling
use tokio::net::TcpStream; // Importing the TcpStream from the tokio crate for TCP connection handling
use tokio::sync::mpsc; // Importing multi-producer, single-consumer (mpsc) channels from the tokio crate

// Function to scan ports for a given subdomain with a specified level of concurrency
pub async fn scan_ports(concurrency: usize, subdomain: Subdomain) -> Subdomain {
    let mut ret = subdomain.clone(); // Clone the subdomain to retain the original
    let socket_addresses: Vec<SocketAddr> = format!("{}:1024", subdomain.domain)
        .to_socket_addrs() // Convert the domain to socket addresses
        .expect("port scanner: Creating socket address") // Expect a valid socket address, otherwise panic
        .collect();

    if socket_addresses.is_empty() { // Check if there are no socket addresses
        return subdomain; // Return the original subdomain if no addresses are found
    }

    let socket_address = socket_addresses[0]; // Use the first socket address

    // Concurrent stream method 3: using channels
    let (input_tx, input_rx) = mpsc::channel(concurrency); // Create a channel for input ports with specified concurrency
    let (output_tx, output_rx) = mpsc::channel(concurrency); // Create a channel for output ports with specified concurrency

    // Spawn a task to send the most common ports to the input channel
    tokio::spawn(async move {
        for port in MOST_COMMON_PORTS_100 {
            let _ = input_tx.send(*port).await; // Send each port to the input channel
        }
    });

    let input_rx_stream = tokio_stream::wrappers::ReceiverStream::new(input_rx); // Create a stream from the input receiver
    input_rx_stream
        .for_each_concurrent(concurrency, |port| { // Process ports concurrently
            let output_tx = output_tx.clone(); // Clone the output transmitter
            async move {
                let port = scan_port(socket_address, port).await; // Scan the port
                if port.is_open {
                    let _ = output_tx.send(port).await; // Send the open port to the output channel
                }
            }
        })
        .await;
    
    drop(output_tx); // Close the output channel

    let output_rx_stream = tokio_stream::wrappers::ReceiverStream::new(output_rx); // Create a stream from the output receiver
    ret.open_ports = output_rx_stream.collect().await; // Collect the open ports and assign them to the subdomain

    ret // Return the subdomain with the scanned open ports
}

// Function to scan an individual port for a given socket address
async fn scan_port(mut socket_address: SocketAddr, port: u16) -> Port {
    let timeout = Duration::from_secs(3); // Set a timeout duration of 3 seconds
    socket_address.set_port(port); // Set the port for the socket address

    let is_open = matches!( // Check if the port is open using a timeout
        tokio::time::timeout(timeout, TcpStream::connect(&socket_address)).await,
        Ok(Ok(_)),
    );

    Port {
        port, // The port number
        is_open, // Whether the port is open
    }
}

// Explanation of the modules used:
// - crate::common_ports: This module contains a list of the 100 most common ports that are typically scanned to check if they are open.
// - crate::model: This module contains the data structures Port and Subdomain. Port represents a network port and its state (open or closed), while Subdomain represents a subdomain with associated open ports.
// - futures::StreamExt: This module provides additional methods for working with streams, which are essential for handling asynchronous data processing.
// - std::net and std::time: These standard library modules provide functionality for handling network socket addresses and time durations, respectively.
// - tokio::net::TcpStream: This module provides the TcpStream struct for asynchronous TCP connections, which is used to check if a port is open by attempting to connect to it.
// - tokio::sync::mpsc: This module provides multi-producer, single-consumer (mpsc) channels, which are used for passing ports to be scanned and collecting the results concurrently.