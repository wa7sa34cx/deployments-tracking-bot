pub mod apps;
pub mod auth;

use reqwest::Client;

pub struct DigitalOcean {
    token: String,
    client: Client,
}
