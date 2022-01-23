//! Module for retrieving list of all applications in the account with last deployment.

use reqwest::header;
use serde_derive::*;

use crate::digitalocean::DigitalOcean;

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

impl DigitalOcean {
    /// Gets list of apps
    pub async fn get_apps(&self) -> anyhow::Result<Vec<App>> {
        let json_res = self.get_json_res().await?;

        let apps: Vec<App> = json_res
            .apps
            .into_iter()
            .map(|json_app| App {
                id: json_app.id,
                name: json_app.spec.name,
            })
            .collect();

        Ok(apps)
    }

    async fn get_json_res(&self) -> anyhow::Result<JsonResponse> {
        let json_res = self
            .client
            .get("https://api.digitalocean.com/v2/apps")
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::AUTHORIZATION, &format!("Bearer {}", self.token))
            .send()
            .await?
            .json::<JsonResponse>()
            .await?;

        Ok(json_res)
    }
}
