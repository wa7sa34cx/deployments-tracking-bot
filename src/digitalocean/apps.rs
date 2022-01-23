//! Retrieving list of all applications in the account.

use reqwest::{header, StatusCode};
use serde_derive::*;

use crate::digitalocean::DigitalOcean;

/// App info
#[derive(Debug)]
pub struct App {
    pub id: String,
    pub name: String,
}

// https://docs.digitalocean.com/reference/api/api-reference/#operation/list_apps
#[derive(Debug, Deserialize)]
pub struct JsonResponse {
    pub apps: Vec<JsonApp>,
}

#[derive(Debug, Deserialize)]
pub struct JsonApp {
    pub id: String,
    pub spec: Spec,
}

#[derive(Debug, Deserialize)]
pub struct Spec {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub id: String,
    pub message: String,
}

impl DigitalOcean {
    /// Gets list of apps
    pub async fn get_apps(&self) -> anyhow::Result<Vec<App>> {
        let json = self.get_json().await?;

        let apps: Vec<App> = json
            .apps
            .into_iter()
            .map(|json_app| App {
                id: json_app.id,
                name: json_app.spec.name,
            })
            .collect();

        Ok(apps)
    }

    // Gets json data from DigitalOcean API
    async fn get_json(&self) -> anyhow::Result<JsonResponse> {
        let res = self
            .client
            .get("https://api.digitalocean.com/v2/apps")
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::AUTHORIZATION, &format!("Bearer {}", self.token))
            .send()
            .await?;
        // .json::<JsonResponse>()
        // .await?;

        if res.status() != StatusCode::OK {
            let json = res.json::<ErrorResponse>().await?;
            return Err(anyhow::anyhow!(json.message));
        }

        let json = res.json::<JsonResponse>().await?;

        Ok(json)
    }
}
