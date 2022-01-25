//! DigitalOcean initialization

use reqwest::{header, Client, StatusCode};
use serde_derive::Deserialize;

use crate::digitalocean::{error::ErrorResponse, DigitalOcean};

/// Account info
// https://docs.digitalocean.com/reference/api/api-reference/#operation/get_user_information
#[derive(Debug, Deserialize)]
pub struct Account {
    pub status: Status,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Active,
    Warning,
    Locked,
}

impl DigitalOcean {
    /// Creates new DigitalOcean instance
    ///
    /// # Panics
    ///
    /// Panics if the DO_TOKEN variable are not specified in environment
    pub fn from_env() -> Self {
        let token = dotenv::var("DO_TOKEN").unwrap();

        // Create keep-alive HTTP connection pool
        let client = Client::new();

        Self { token, client }
    }

    // Initializes working with DigitalOcean API, checks account status
    pub async fn init(self) -> anyhow::Result<Self> {
        let res = self
            .client
            .get("https://api.digitalocean.com/v2/account")
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::AUTHORIZATION, &format!("Bearer {}", &self.token))
            .send()
            .await?;

        if res.status() != StatusCode::OK {
            let json = res.json::<ErrorResponse>().await?;
            return Err(anyhow::anyhow!(json.error()));
        }

        let json = res.json::<Account>().await?;

        // Check account status
        match json.status {
            Status::Active => { 
                log::debug!("working with DigitalOcean API has been successfully initialized");
                Ok(self)
            },
            Status::Warning => Err(anyhow::anyhow!("your account is in warning status")),
            Status::Locked => Err(anyhow::anyhow!("your account is loked")),
        }
    }
}
