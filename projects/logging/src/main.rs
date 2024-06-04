use log::{info, trace, warn, error, debug};
use env_logger;
use std::{
    env,
};

fn main() {
    // Initialize the logger
    env::set_var("RUST_LOG", "info,trust_dns_proto=error");
    env_logger::init();

    // Only having the below without env_logger wont print anything
    info!("message with info level");
    error!("message with error level");
    debug!("message with debug level");
    trace!("message with trace level");
    warn!("message with warn level");
}