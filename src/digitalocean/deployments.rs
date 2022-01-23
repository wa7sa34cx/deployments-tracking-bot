//! Retrieving a list of all deployments of an app.

use reqwest::{header, StatusCode};
use serde_derive::Deserialize;

use crate::digitalocean::{DigitalOcean, error::ErrorResponse};

// https://docs.digitalocean.com/reference/api/api-reference/#operation/list_deployments
#[derive(Debug, Deserialize)]
pub struct JsonResponse {
    pub deployments: Vec<Deployment>,
}

#[derive(Debug, Deserialize)]
pub struct Deployment {
    pub id: String,
    pub spec: Spec,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct Spec {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ActiveDeployment {
    pub id: String,
    pub cause: String,
    pub progress: Progress,
    pub phase: String,
}

#[derive(Debug, Deserialize)]
pub struct Progress {
    pub success_steps: u16,
    pub total_steps: u16,
    pub summary_steps: Option<Vec<SummarySteps>>,
}

#[derive(Debug, Deserialize)]
pub struct SummarySteps {
    pub status: String,
    pub reason: Option<Reason>,
    pub message_base: String,
}

#[derive(Debug, Deserialize)]
pub struct Reason {
    pub code: String,
    pub message: String,
}

impl DigitalOcean {
    pub fn deployments(&self, app_id: impl Into<String>) -> Deployments {
        Deployments {
            response: format!("Response: {}", app_id.into()),
        }
    }
}

