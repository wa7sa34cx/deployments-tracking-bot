//! Creating AppsHandler.

use crate::digitalocean::DigitalOcean;

pub mod get;

/// Apps handler
#[derive(Debug)]
pub struct AppsHandler {
    pub digitalocean: DigitalOcean,
    pub api_url: String,
}

impl DigitalOcean {
    /// Creates apps endpoint config
    pub fn apps(&self) -> AppsHandler {
        let api_url = format!("{}{}", self.api_url, "apps");

        AppsHandler {
            digitalocean: self.clone(),
            api_url,
        }
    }
}
