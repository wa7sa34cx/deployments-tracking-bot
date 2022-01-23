pub mod apps;
pub mod auth;

use reqwest::Client;

#[derive(Debug)]
pub struct DigitalOcean {
    token: String,
    client: Client,
}
