// Importing the Deserialize trait from Serde for deserializing JSON data
use serde::Deserialize;

// Define the Subdomain struct
// #[derive(Debug, Clone)] automatically implements the Debug and Clone traits for Subdomain
#[derive(Debug, Clone)]
pub struct Subdomain {
    // The domain field holds the subdomain name as a String
    pub domain: String,
    // The open_ports field holds a vector of Port structs representing the open ports for the subdomain
    pub open_ports: Vec<Port>,
}

// Define the Port struct
// #[derive(Debug, Clone)] automatically implements the Debug and Clone traits for Port
#[derive(Debug, Clone)]
pub struct Port {
    // The port field holds the port number as an unsigned 16-bit integer
    pub port: u16,
    // The is_open field indicates whether the port is open (true) or closed (false)
    pub is_open: bool,
}

// Define the CrtShEntry struct
// #[derive(Debug, Deserialize, Clone)] automatically implements the Debug, Deserialize, and Clone traits for CrtShEntry
#[derive(Debug, Deserialize, Clone)]
pub struct CrtShEntry {
    // The name_value field holds the name value from the crt.sh response as a String
    pub name_value: String,
}