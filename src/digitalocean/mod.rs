//! DigitalOcean Module.

use reqwest::Client;

pub mod apps;
pub mod deployments;
pub mod error;
pub mod init;

/// Main DigitalOcean struct
#[derive(Debug)]
pub struct DigitalOcean {
    token: String,
    client: Client,
}
