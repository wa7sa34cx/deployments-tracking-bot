//! Retrieving a list of all deployments of an app.

use chrono::{DateTime, Utc};
use humantime::format_duration;
use reqwest::{header, StatusCode};
use serde_derive::Deserialize;

use crate::digitalocean::{apps::App, error::ErrorResponse, DigitalOcean};

/// Deployment info
#[derive(Debug)]
pub struct Deployment {
    pub id: String,
    pub cause: String,
    pub phase: Phase,
    pub took_time: String,
    pub error: DeploymentError,
}

#[derive(Debug, Default)]
pub struct DeploymentError {
    pub message: Option<String>,
    pub action: Option<String>,
}

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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Phase {
    Unknown,
    PendingBuild,
    Building,
    PendingDeploy,
    Deployng,
    Active,
    Superseded,
    Error,
    Canceled,
}

#[derive(Debug, Deserialize)]
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

impl DigitalOcean {
    /// Gets a list of all deployments of the app
    pub async fn get_deployments(&self, app: &App) -> anyhow::Result<Vec<Deployment>> {
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
                Phase::Active | Phase::Superseded | Phase::Error | Phase::Canceled => true,
                _ => false,
            })
            .map(|d| Deployment {
                id: d.id.unwrap(),
                cause: d.cause.unwrap(),
                phase: d.phase.unwrap(),
                took_time: took_time(d.created_at.unwrap(), d.updated_at.unwrap())
                    .unwrap_or("unknown".to_string()),
                error: create_error(d.progress).unwrap_or(DeploymentError::default()),
            })
            .collect();

        Ok(deployments)
    }
}

// Gets json data from DigitalOcean API
async fn get_json(digitalocean: &DigitalOcean, app: &App) -> anyhow::Result<JsonResponse> {
    let res = digitalocean
        .client
        .get(&format!(
            "https://api.digitalocean.com/v2/apps/{}/deployments?per_page=5",
            app.id
        ))
        .header(header::CONTENT_TYPE, "application/json")
        .header(
            header::AUTHORIZATION,
            &format!("Bearer {}", digitalocean.token),
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

#[derive(thiserror::Error, Debug)]
enum CreateError {
    #[error("progress is None")]
    ProgressNone,
    #[error("summary_steps is None")]
    SummaryStepsNone,
    #[error("summary_steps contains no elements")]
    NoElements,
}

// Creates DeploymentError from Progress
fn create_error(progress: Option<Progress>) -> anyhow::Result<DeploymentError> {
    let summary = progress
        .ok_or_else(|| CreateError::ProgressNone)?
        .summary_steps
        .ok_or_else(|| CreateError::SummaryStepsNone)?
        .into_iter()
        .nth(0)
        .ok_or_else(|| CreateError::NoElements)?;

    let message = summary.reason.map_or(None, |r| r.message);
    let action = summary.message_base;

    Ok(DeploymentError { message, action })
}
