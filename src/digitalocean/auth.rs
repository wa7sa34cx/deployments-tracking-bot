//! Module for authentication with OAuth

use crate::digitalocean::DigitalOcean;
use reqwest::Client;

impl DigitalOcean {
    /// Creates new instance with token and client
    pub fn auth(token: impl Into<String>, client: Client) -> Self {
        Self {
            token: token.into(),
            client,
        }
    }
}
