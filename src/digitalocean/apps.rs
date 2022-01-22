//! List all apps on account.

use crate::digitalocean::DigitalOcean;

#[derive(Debug)]
pub struct App {
    pub id: String,
    pub name: String,
    pub created_at: String,

}

impl DigitalOcean {
    /// Gets list of apps
    pub fn apps(&self) -> Vec<App> {
        vec![App {
            internal: "1".to_string(),
        },
        App {
            internal: "2".to_string(),
        }]
    }
}