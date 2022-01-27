//! DigitalOcean initialization.

use reqwest::{header, StatusCode};
use serde_derive::Deserialize;
use std::sync::Arc;

use crate::digitalocean::{error::ErrorResponse, DigitalOcean, DigitalOceanConfig};

// https://docs.digitalocean.com/reference/api/api-reference/#operation/get_user_information
#[derive(Debug, Deserialize)]
pub struct JsonResponse {
    pub account: Account,
}

// Account info
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

impl DigitalOceanConfig {
    // Initializes working with DigitalOcean API, checks account status
    pub async fn init(self) -> anyhow::Result<DigitalOcean> {
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

        let json = res.json::<JsonResponse>().await?;

        // Check account status
        match json.account.status {
            Status::Active => {
                log::debug!("working with DigitalOcean API has been successfully initialized");
                Ok(DigitalOcean(Arc::new(DigitalOceanConfig {
                    api_url: self.api_url,
                    token: self.token,
                    client: self.client,
                })))
            }
            Status::Warning => Err(anyhow::anyhow!("your account is in warning status")),
            Status::Locked => Err(anyhow::anyhow!("your account is loked")),
        }
    }
}
