pub mod apps;
pub mod auth;
pub mod deployments;

use reqwest::Client;

pub struct DigitalOcean {
    token: String,
    client: Client,
}
