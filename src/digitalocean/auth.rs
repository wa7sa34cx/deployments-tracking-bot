//! Module for authentication with OAuth

use reqwest::Client;
use crate::digitalocean::DigitalOcean;


impl DigitalOcean {
    /// Creates new instance with token and client
    pub fn auth(token: impl Into<String>, client: Client) -> Self {
        Self {
            token: token.into(),
            client,
        }
    }
}
