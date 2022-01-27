//! Creating DeploymentsHandler.

use crate::digitalocean::DigitalOcean;

pub mod get;

/// Deployments handler
#[derive(Debug)]
pub struct DeploymentsHandler {
    pub digitalocean: DigitalOcean,
    pub api_url: String,
}

impl DigitalOcean {
    /// Creates deployments endpoint handler
    pub fn deployments(&self) -> DeploymentsHandler {
        let api_url = format!("{}{}", self.api_url, "apps/{app_id}/deployments?per_page=5");

        DeploymentsHandler {
            digitalocean: self.clone(),
            api_url,
        }
    }
}
