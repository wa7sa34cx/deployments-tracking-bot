use chrono::{DateTime, Utc};
use serde_derive::Deserialize;

use crate::digitalocean::models::app::App;

/// Deployment info
#[derive(Debug)]
pub struct Deployment {
    pub id: String,
    pub app: App,
    pub cause: String,
    pub phase: Phase,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub took_time: String,
    pub error: DeploymentError,
}

#[derive(Debug, Default)]
pub struct DeploymentError {
    pub message: Option<String>,
    pub action: Option<String>,
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
