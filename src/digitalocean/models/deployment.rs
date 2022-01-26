use chrono::{DateTime, Utc};
use serde_derive::Deserialize;
use std::fmt;

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
    Deploying,
    Active,
    Superseded,
    Error,
    Canceled,
}

impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let printable = match *self {
            Phase::Unknown => "🤷‍♂️ Unknown",
            Phase::PendingBuild => "🏗 Pending build",
            Phase::Building => "🏗 Building",
            Phase::PendingDeploy => "🏗 Pending deploy",
            Phase::Deploying => "🏗 Deploying",
            Phase::Active => "✅ Live",
            Phase::Superseded => "🔷 Superseded",
            Phase::Error => "🚨 Faild",
            Phase::Canceled => "❌ Canceled",
        };

        write!(f, "{}", printable)
    }
}


// impl Deployment {
//     pub fn message(&self) -> String {
//         format!("🏗 New deployment has been detected\n
//         App: {}\n
//         Status: {}\n
//         Cause: {}\n
//         Updated at: {}\n
//         Took time: {}\n\n
//         ", self.app.name)
//     }
// }
