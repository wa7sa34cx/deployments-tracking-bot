//! Creating DeploymentsHandler instance.

use crate::digitalocean::{DigitalOcean, DigitalOceanConfig};

pub mod get;

/// DigitalOcean deployments endpoint config
#[derive(Debug)]
pub struct DeploymentsHandler<'a> {
    pub config: &'a DigitalOceanConfig,
    pub url: &'a str,
}

impl DigitalOcean {
    /// Creates deployments endpoint config
    pub fn deployments(&self) -> DeploymentsHandler {
        DeploymentsHandler {
            config: &self,
            url: "https://api.digitalocean.com/v2/apps/{app_id}/deployments?per_page=5",
        }
    }
}
