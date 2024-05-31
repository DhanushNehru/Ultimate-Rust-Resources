// Import necessary modules and items
use crate::{
    common_ports::MOST_COMMON_PORTS_100, // Import the list of most common ports
    model::{Port, Subdomain}, // Import the Port and Subdomain structs
};
use rayon::prelude::*; // Import Rayon for parallel iteration
use std::net::{SocketAddr, ToSocketAddrs}; // Import networking utilities
use std::{net::TcpStream, time::Duration}; // Import TcpStream and Duration for handling TCP connections and timeouts

// Function to scan ports for a given subdomain
pub fn scan_ports(mut subdomain: Subdomain) -> Subdomain {
    // Create a vector of socket addresses for the subdomain
    let socket_addresses: Vec<SocketAddr> = format!("{}:1024", subdomain.domain) // Create a socket address string with port 1024 (placeholder)
        .to_socket_addrs() // Convert the string to socket addresses
        .expect("port scanner: Creating socket address") // Handle errors in address conversion
        .collect(); // Collect the results into a vector

    // If no socket addresses are found, return the original subdomain
    if socket_addresses.is_empty() {
        return subdomain;
    }

    // Scan the most common ports in parallel
    subdomain.open_ports = MOST_COMMON_PORTS_100
        .into_par_iter() // Convert the list of common ports to a parallel iterator
        .map(|port| scan_port(socket_addresses[0], *port)) // Scan each port and get the result
        .filter(|port| port.is_open) // Filter out closed ports
        .collect(); // Collect the open ports into the subdomain's open_ports vector

    subdomain // Return the subdomain with the updated open_ports field
}

// Function to scan an individual port on a given socket address
fn scan_port(mut socket_address: SocketAddr, port: u16) -> Port {
    let timeout = Duration::from_secs(3); // Set a timeout duration of 3 seconds
    socket_address.set_port(port); // Set the port for the socket address

    // Attempt to connect to the socket address with the specified timeout
    let is_open = TcpStream::connect_timeout(&socket_address, timeout).is_ok();

    // Return a Port struct with the port number and whether it is open
    Port { port, is_open }
}