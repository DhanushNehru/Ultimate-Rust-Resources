use crate::{
    model::{CrtShEntry, Subdomain}, // Importing models CrtShEntry and Subdomain from the model module
    Error, // Importing custom Error type from the current crate
};
use reqwest::blocking::Client; // Importing Reqwest blocking Client for HTTP requests
use std::{collections::HashSet, time::Duration}; // Importing HashSet for deduplication and Duration for setting timeouts
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts}, // Importing DNS resolver configuration and options
    Resolver, // Importing DNS Resolver
};

// Function to enumerate subdomains using HTTP client and target domain
pub fn enumerate(http_client: &Client, target: &str) -> Result<Vec<Subdomain>, Error> {
    // Send a GET request to crt.sh and parse the response as JSON into a vector of CrtShEntry
    // The reason for parsing the response into CrtShEntry is to facilitate the structured handling and processing of data obtained from the crt.sh API. By parsing the JSON response into a structured format
    let entries: Vec<CrtShEntry> = http_client
        .get(&format!("https://crt.sh/?q=%25.{}&output=json", target))
        .send()?
        .json()?;

    // Clean and deduplicate results
    let mut subdomains: HashSet<String> = entries
        .into_iter() // Iterate over each entry
        .flat_map(|entry| {
            // Split the name_value field by newline, trim whitespace, and collect into a vector of strings
            entry
                .name_value
                .split('\n')
                .map(|subdomain| subdomain.trim().to_string())
                .collect::<Vec<String>>()
        })
        .filter(|subdomain: &String| subdomain != target) // Filter out entries matching the target
        .filter(|subdomain: &String| !subdomain.contains('*')) // Filter out wildcard entries
        .collect(); // Collect unique subdomains into a HashSet
    subdomains.insert(target.to_string()); // Insert the target domain itself

    // Convert the HashSet of subdomains into a vector of Subdomain structs, filter out non-resolving domains
    let subdomains: Vec<Subdomain> = subdomains
        .into_iter()
        .map(|domain| Subdomain {
            domain, // Populate the domain field
            open_ports: Vec::new(), // Initialize the open_ports field as an empty vector
        })
        .filter(resolves) // Filter out domains that do not resolve
        .collect(); // Collect the results into a vector

    Ok(subdomains) // Return the vector of Subdomain structs
}

// Function to check if a domain resolves (has valid DNS records)
pub fn resolves(domain: &Subdomain) -> bool {
    let mut opts = ResolverOpts::default(); // Get default resolver options
    opts.timeout = Duration::from_secs(4); // Set a custom timeout of 4 seconds

    // Create a DNS resolver with default configuration and the specified options
    let dns_resolver = Resolver::new(
        ResolverConfig::default(),
        opts,
    )
    .expect("subdomain resolver: building DNS client"); // Panic if DNS resolver creation fails

    // Check if the DNS lookup for the domain is successful
    dns_resolver.lookup_ip(domain.domain.as_str()).is_ok()
}
          
