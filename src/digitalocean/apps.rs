//! Module for retrieving list of all applications in the account with last deployment.

use chrono::NaiveDateTime;
use reqwest::header;
use serde_derive::*;

use crate::digitalocean::DigitalOcean;

#[derive(Debug)]
pub struct App {
    pub id: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// https://docs.digitalocean.com/reference/api/api-reference/#operation/list_apps
#[derive(Deserialize)]
pub struct JsonResponse {
    apps: Vec<JsonApp>
}

impl DigitalOcean {
    /// Gets list of apps
    pub fn get_apps(&self) -> Vec<App> {
        vec![App {
            internal: "1".to_string(),
        },
        App {
            internal: "2".to_string(),
        }]
    }
}


// Gets list all Apps
async fn get_user_data(
    access_token: impl Into<String>,
    client: &Client,
) -> anyhow::Result<UserData> {
    let user_data = client
        .get("https://discord.com/api/users/@me")
        .header(
            header::AUTHORIZATION,
            &format!("Bearer {}", access_token.into()),
        )
        .send()
        .await?
        .json::<UserData>()
        .await?;

    Ok(user_data)
}