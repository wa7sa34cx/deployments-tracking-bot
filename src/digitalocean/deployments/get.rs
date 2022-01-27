//! Retrieving a list of all deployments of an app.

use chrono::{DateTime, Utc};
use humantime::format_duration;
use reqwest::{header, StatusCode};
use serde_derive::Deserialize;

use crate::digitalocean::{
    deployments::DeploymentsHandler,
    error::ErrorResponse,
    models::{
        app::App,
        deployment::{Deployment, DeploymentError, Phase},
    },
};

// https://docs.digitalocean.com/reference/api/api-reference/#operation/list_deployments
#[derive(Debug, Deserialize)]
pub struct JsonResponse {
    pub deployments: Option<Vec<JsonDeployment>>,
}

#[derive(Debug, Deserialize)]
pub struct JsonDeployment {
    pub id: Option<String>,
    pub cause: Option<String>,
    pub phase: Option<Phase>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub progress: Option<Progress>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Progress {
    pub success_steps: Option<u32>,
    pub error_steps: Option<u32>,
    pub total_steps: Option<u32>,
    pub summary_steps: Option<Vec<SummarySteps>>,
}

#[derive(Debug, Deserialize)]
pub struct SummarySteps {
    pub reason: Option<Reason>,
    pub message_base: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Reason {
    pub code: Option<String>,
    pub message: Option<String>,
}

// Creates DeploymentError from Progress
impl Progress {
    fn create_error(&self) -> anyhow::Result<DeploymentError> {
        let summary = self
            .summary_steps
            .ok_or_else(|| anyhow::anyhow!("summary_steps is None"))?
            .into_iter()
            .nth(0)
            .ok_or_else(|| anyhow::anyhow!("summary_steps contains no elements"))?;

        let message = summary.reason.map_or(None, |r| r.message);
        let action = summary.message_base;

        Ok(DeploymentError { message, action })
    }
}

impl DeploymentsHandler {
    /// Gets a list of all deployments of the app
    pub async fn get(&self, app: &App) -> anyhow::Result<Vec<Deployment>> {
        let json = get_json(self, app).await?;

        if json.deployments.is_none() {
            return Err(anyhow::anyhow!(
                "there are no deployments for this application 🤷‍♂️"
            ));
        }

        let deployments: Vec<Deployment> = json
            .deployments
            .unwrap()
            .into_iter()
            .filter(|d| {
                d.id.is_some()
                    && d.cause.is_some()
                    && d.phase.is_some()
                    && d.created_at.is_some()
                    && d.updated_at.is_some()
            })
            .filter(|d| match d.phase.as_ref().unwrap() {
                Phase::Active | Phase::Error => true,
                _ => false,
            })
            .map(|d| Deployment {
                id: d.id.unwrap(),
                app: app.clone(),
                cause: d.cause.unwrap(),
                phase: d.phase.unwrap(),
                created_at: d.created_at.unwrap(),
                updated_at: d.updated_at.unwrap(),
                took_time: took_time(d.created_at.unwrap(), d.updated_at.unwrap())
                    .unwrap_or("unknown".to_string()),
                error: d
                    .progress
                    .unwrap_or(Progress::default())
                    .create_error()
                    .unwrap_or(DeploymentError::default()),
            })
            .collect();

        if deployments.len() == 0 {
            return Err(anyhow::anyhow!(
                "there are no deployments with required fields 🤷‍♂️"
            ));
        }

        Ok(deployments)
    }
}

// Gets json data from DigitalOcean API
async fn get_json(handler: &DeploymentsHandler, app: &App) -> anyhow::Result<JsonResponse> {
    let url = handler.api_url.replace("{app_id}", &app.id);

    let res = handler
        .digitalocean
        .client
        .get(url)
        .header(header::CONTENT_TYPE, "application/json")
        .header(
            header::AUTHORIZATION,
            &format!("Bearer {}", handler.digitalocean.token),
        )
        .send()
        .await?;

    if res.status() != StatusCode::OK {
        let json = res.json::<ErrorResponse>().await?;
        return Err(anyhow::anyhow!(json.error()));
    }

    let json = res.json::<JsonResponse>().await?;

    Ok(json)
}

// Calculates the took time
fn took_time(start: DateTime<Utc>, end: DateTime<Utc>) -> anyhow::Result<String> {
    Ok(format_duration((end - start).to_std()?).to_string())
}
