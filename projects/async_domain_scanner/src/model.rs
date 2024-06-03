use serde::Deserialize; // Importing the Deserialize trait from the serde crate for deserializing JSON data

// Define the Subdomain struct to represent a subdomain and its associated open ports
#[derive(Debug, Clone)]
pub struct Subdomain {
    pub domain: String, // The domain name of the subdomain
    pub open_ports: Vec<Port>, // A vector of open ports on the subdomain
}

// Define the Port struct to represent a network port and its state (open or closed)
#[derive(Debug, Clone)]
pub struct Port {
    pub port: u16, // The port number
    pub is_open: bool, // A boolean indicating whether the port is open
}

// Define the CrtShEntry struct to represent an entry from the crt.sh database
#[derive(Debug, Deserialize, Clone)]
pub struct CrtShEntry {
    pub name_value: String, // The name value from the crt.sh entry, which contains subdomain names
}

// Explanation of the modules used:
// - serde::Deserialize: The Deserialize trait from the serde crate is used to automatically convert JSON data into Rust structs. In this case, it is used to convert JSON data from the crt.sh API into CrtShEntry structs.
// - #[derive(Debug, Clone)]: These derive macros automatically implement the Debug and Clone traits for the structs, allowing them to be easily printed for debugging and cloned to create copies.
// - #[derive(Deserialize)]: This derive macro automatically implements the Deserialize trait for the CrtShEntry struct, enabling it to be deserialized from JSON data.