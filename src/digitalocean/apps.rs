//! Creating DigitalOceanApps instance.

use crate::digitalocean::{DigitalOcean, DigitalOceanConfig};

pub mod get;

#[derive(Debug)]
pub struct AppsHandler<'a> {
    pub config: &'a DigitalOceanConfig,
    pub url: &'a str,
}

impl DigitalOcean {
    /// Creates apps endpoint config
    pub fn apps(&self) -> AppsHandler {
        AppsHandler {
            config: &self,
            url: "https://api.digitalocean.com/v2/apps",
        }
    }
}
