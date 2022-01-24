//! DigitalOcean Module.

use reqwest::Client;

pub mod apps;
pub mod auth;
pub mod deployments;
pub mod error;

/// Main DigitalOcean struct
#[derive(Debug)]
pub struct DigitalOcean {
    token: String,
    client: Client,
}
