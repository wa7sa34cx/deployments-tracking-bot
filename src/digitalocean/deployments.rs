//! Creating DeploymentsHandler instance.

use crate::digitalocean::DigitalOcean;

pub mod get;

/// DigitalOcean deployments endpoint config
#[derive(Debug)]
pub struct DeploymentsHandler {
    pub digitalocean: DigitalOcean,
    pub url: &'static str,
}

impl DigitalOcean {
    /// Creates deployments endpoint config
    pub fn deployments(&self) -> DeploymentsHandler {
        DeploymentsHandler {
            digitalocean: self.clone(),
            url: "https://api.digitalocean.com/v2/apps/{app_id}/deployments?per_page=5",
        }
    }
}
