//! Retrieving a list of all applications in the account.

use reqwest::{header, StatusCode};
use serde_derive::Deserialize;

use crate::digitalocean::{
    apps::{App, Apps},
    error::ErrorResponse,
};

// https://docs.digitalocean.com/reference/api/api-reference/#operation/list_apps
#[derive(Debug, Deserialize)]
pub struct JsonResponse {
    pub apps: Option<Vec<JsonApp>>,
}

#[derive(Debug, Deserialize)]
pub struct JsonApp {
    pub id: Option<String>,
    pub spec: Spec,
}

#[derive(Debug, Deserialize)]
pub struct Spec {
    pub name: String,
}

impl<'a> Apps<'a> {
    /// Gets list of apps
    pub async fn get(&self) -> anyhow::Result<Vec<App>> {
        let json = get_json(self).await?;

        if json.apps.is_none() {
            return Err(anyhow::anyhow!(
                "there are no applications in the account 🤷‍♂️"
            ));
        }

        let apps: Vec<App> = json
            .apps
            .unwrap()
            .into_iter()
            .filter(|j| j.id.is_some())
            .map(|j| App {
                id: j.id.unwrap(),
                name: j.spec.name,
            })
            .collect();

        if apps.len() == 0 {
            return Err(anyhow::anyhow!("there are no applications with id 🤷‍♂️"));
        }

        Ok(apps)
    }
}

// Gets json data from DigitalOcean API
async fn get_json<'a>(apps: &Apps<'a>) -> anyhow::Result<JsonResponse> {
    let res = apps
        .client
        .get(apps.url)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::AUTHORIZATION, &format!("Bearer {}", apps.token))
        .send()
        .await?;

    if res.status() != StatusCode::OK {
        let json = res.json::<ErrorResponse>().await?;
        return Err(anyhow::anyhow!(json.error()));
    }

    let json = res.json::<JsonResponse>().await?;

    Ok(json)
}
