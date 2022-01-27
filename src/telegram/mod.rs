//! Telegram module.
//!
//! A new era of messaging 💬

use reqwest::Client;
use std::ops::Deref;
use std::sync::Arc;

pub mod init;
pub mod message;

static TG_BOT_API_URL: &str = "https://api.telegram.org/bot";

/// Main Telegram struct
#[derive(Debug)]
pub struct Telegram(Arc<TelegramConfig>);

/// Telegram configuration
#[derive(Debug)]
pub struct TelegramConfig {
    api_url: String,
    chat_id: i64,
    client: Client,
}

impl Clone for Telegram {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl Deref for Telegram {
    type Target = TelegramConfig;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Telegram {
    /// Creates TelegramConfig from environment
    ///
    /// # Panics
    ///
    /// Panics if the TG_TOKEN variable are not specified in environment
    pub fn from_env() -> TelegramConfig {
        let token = dotenv::var("TG_TOKEN").unwrap();

        // https://core.telegram.org/bots/api#making-requests
        let api_url = format!("{}{}/", TG_BOT_API_URL, &token);

        let chat_id = dotenv::var("TG_CHAT_ID")
            .unwrap()
            .parse::<i64>()
            .unwrap();

        // Create keep-alive HTTP connection pool
        let client = Client::new();

        TelegramConfig { api_url, chat_id, client }
    }
}
