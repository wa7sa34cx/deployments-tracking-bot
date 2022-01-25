//! Creating Apps instance.

use reqwest::Client;

use crate::digitalocean::DigitalOcean;

pub mod get;

#[derive(Debug)]
pub struct Apps<'a> {
    pub token: &'a str,
    pub client: &'a Client,
    pub url: &'a str,
}

impl DigitalOcean {
    /// Creates apps endpoint
    pub fn apps(&self) -> Apps {
        Apps {
            token: &self.token,
            client: &self.client,
            url: "https://api.digitalocean.com/v2/apps",
        }
    }
}

/// App info
#[derive(Debug)]
pub struct App {
    pub id: String,
    pub name: String,
}
