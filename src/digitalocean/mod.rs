//! DigitalOcean Module.

use reqwest::Client;
use std::ops::Deref;
use std::sync::Arc;

pub mod apps;
pub mod deployments;
pub mod error;
pub mod init;
pub mod models;

static DO_API_URL: &str = "https://api.digitalocean.com/v2/";

/// Main DigitalOcean struct
#[derive(Debug)]
pub struct DigitalOcean(Arc<DigitalOceanConfig>);

/// DigitalOcean configuration
#[derive(Debug)]
pub struct DigitalOceanConfig {
    api_url: &'static str,
    token: String,
    client: Client,
}

impl Clone for DigitalOcean {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl Deref for DigitalOcean {
    type Target = DigitalOceanConfig;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DigitalOcean {
    /// Creates DigitalOceanConfig from environment
    ///
    /// # Panics
    ///
    /// Panics if the DO_TOKEN variable are not specified in environment
    pub fn from_env() -> DigitalOceanConfig {
        let api_url = DO_API_URL;
        let token = dotenv::var("DO_TOKEN").unwrap();

        // Create keep-alive HTTP connection pool
        let client = Client::new();

        DigitalOceanConfig { api_url, token, client }
    }
}
