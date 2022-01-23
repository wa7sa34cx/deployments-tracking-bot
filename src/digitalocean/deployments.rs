//! Module for getting a list of all deployments of the app

use crate::digitalocean::DigitalOcean;

// https://docs.digitalocean.com/reference/api/api-reference/#operation/list_apps
#[derive(Debug, Deserialize)]
pub struct JsonResponse {
    pub apps: Vec<JsonApp>,
}

#[derive(Debug, Deserialize)]
pub struct JsonApp {
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

