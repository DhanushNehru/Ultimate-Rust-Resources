use crate::{
    model::{CrtShEntry, Subdomain}, // Importing CrtShEntry and Subdomain models from the model module
    Error, // Importing the custom Error type from the error module
};
use futures::stream; // Importing the futures stream module for handling asynchronous streams
use futures::StreamExt; // Importing StreamExt for additional stream methods
use reqwest::Client; // Importing the Client from the reqwest crate for making HTTP requests
use std::{collections::HashSet, time::Duration}; // Importing HashSet for unique collections and Duration for time handling
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts}, // Importing DNS resolver configuration options
    name_server::{GenericConnection, GenericConnectionProvider, TokioRuntime}, // Importing connection types for the DNS resolver
    AsyncResolver, // Importing the asynchronous DNS resolver
};

// Defining a type alias for the asynchronous DNS resolver
type DnsResolver = AsyncResolver<GenericConnection, GenericConnectionProvider<TokioRuntime>>;

// Function to enumerate subdomains for a given target
pub async fn enumerate(http_client: &Client, target: &str) -> Result<Vec<Subdomain>, Error> {
    // Making an HTTP GET request to crt.sh to fetch subdomains for the target
    let entries: Vec<CrtShEntry> = http_client
        .get(&format!("https://crt.sh/?q=%25.{}&output=json", target))
        .send()
        .await?
        .json()
        .await?;

    // Setting DNS resolver options with a timeout of 4 seconds
    let mut dns_resolver_opts = ResolverOpts::default();
    dns_resolver_opts.timeout = Duration::from_secs(4);

    // Building an asynchronous DNS resolver
    let dns_resolver = AsyncResolver::tokio(
        ResolverConfig::default(),
        dns_resolver_opts,
    )
    .expect("subdomain resolver: building DNS client");

    // Cleaning and deduplicating results from crt.sh
    let mut subdomains: HashSet<String> = entries
        .into_iter()
        .map(|entry| {
            entry
                .name_value
                .split("\n")
                .map(|subdomain| subdomain.trim().to_string())
                .collect::<Vec<String>>()
        })
        .flatten()
        .filter(|subdomain: &String| subdomain != target)
        .filter(|subdomain: &String| !subdomain.contains("*"))
        .collect();
    subdomains.insert(target.to_string());

    // Converting the HashSet of subdomains to a Vec of Subdomain structs and filtering them using DNS resolution
    let subdomains: Vec<Subdomain> = stream::iter(subdomains.into_iter())
        .map(|domain| Subdomain {
            domain,
            open_ports: Vec::new(),
        })
        .filter_map(|subdomain| {
            let dns_resolver = dns_resolver.clone();
            async move {
                if resolves(&dns_resolver, &subdomain).await {
                    Some(subdomain)
                } else {
                    None
                }
            }
        })
        .collect()
        .await;

    Ok(subdomains)
}

// Function to check if a subdomain resolves using DNS
pub async fn resolves(dns_resolver: &DnsResolver, domain: &Subdomain) -> bool {
    dns_resolver.lookup_ip(domain.domain.as_str()).await.is_ok()
}

// Explanation of the modules used:
// - crate::model: This module contains the CrtShEntry and Subdomain structs. CrtShEntry represents entries from the crt.sh API, and Subdomain represents subdomains with associated open ports.
// - crate::error: This module defines a custom Error type to handle different kinds of errors in a consistent manner.
// - futures::stream and futures::StreamExt: These are used for creating and working with asynchronous streams, which are essential for handling concurrent tasks in an async function.
// - reqwest: This crate is used for making HTTP requests. It simplifies the process of sending and receiving data over HTTP, which is necessary for fetching subdomain data from crt.sh.
// - std::collections::HashSet: This is used to store unique subdomains and eliminate duplicates.
// - std::time::Duration: This is used to specify timeout durations for HTTP requests and DNS resolutions.
// - trust_dns_resolver: This crate provides DNS resolution functionality. It is used to check if subdomains resolve to valid IP addresses, which is a key step in verifying the existence of subdomains.