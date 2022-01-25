//! Creating DigitalOceanApps instance.

use crate::digitalocean::DigitalOcean;

pub mod get;

#[derive(Debug)]
pub struct AppsHandler {
    pub digitalocean: DigitalOcean,
    pub url: &'static str,
}

impl DigitalOcean {
    /// Creates apps endpoint config
    pub fn apps(&self) -> AppsHandler {
        AppsHandler {
            digitalocean: self.clone(),
            url: "https://api.digitalocean.com/v2/apps",
        }
    }
}
