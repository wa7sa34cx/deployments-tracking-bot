use chrono::{DateTime, Utc};
use serde_derive::Deserialize;
use std::{fmt, path::PathBuf};
use tokio::fs;

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
            Phase::Active => "✅ Deployment Live",
            Phase::Superseded => "🔷 Deployment Superseded",
            Phase::Error => "🚨 Deployment Failed",
            Phase::Canceled => "❌ Deployment Canceled",
        };

        write!(f, "{}", printable)
    }
}

/// Type of message
pub enum MsgType {
    Telegram,
}

impl Deployment {
    pub async fn message(&self, msg_type: MsgType) -> anyhow::Result<String> {
        let path = dotenv::var("MSG_PATH").unwrap_or("./messages/".to_string());
        let mut file = PathBuf::from(path);

        match msg_type {
            MsgType::Telegram => file.push("telegram.txt"),
        }

        // Read from the file
        let contents = fs::read_to_string(file).await?;

        // Prepare some replacements
        let error_message = self.error.message.as_deref().unwrap_or_else(|| "");
        let error_action = self
            .error
            .action
            .as_deref()
            .map(|s| format!("{} 🛠", s))
            .unwrap_or_else(|| "".to_string());
        let updated_at = self.updated_at.format("%H:%M:%S %B %d, %Y UTC").to_string();

        let message = contents
            .replace("{app_name}", &self.app.name)
            .replace("{status}", &format!("{}", &self.phase))
            .replace("{cause}", &self.cause)
            .replace("{updated_at}", &updated_at)
            .replace("{took_time}", &self.took_time)
            .replace("{error_message}", &error_message)
            .replace("{error_action}", &error_action)
            .trim()
            .to_string();

        Ok(message)
    }
}
