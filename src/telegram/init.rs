//! Telegram initialization.

use serde_derive::Deserialize;
use std::sync::Arc;

use crate::telegram::{Telegram, TelegramConfig};

// https://core.telegram.org/bots/api#making-requests
#[derive(Debug, Deserialize)]
pub struct JsonResponse {
    pub ok: bool,
    pub result: Option<User>,
    pub description: Option<String>,
}

// https://core.telegram.org/bots/api#user
#[derive(Debug, Deserialize)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
}

impl TelegramConfig {
    // Initializes working with Telegram API, checks bot status
    pub async fn init(self) -> anyhow::Result<Telegram> {
        let url = format!("{}getMe", &self.api_url);

        let json = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<JsonResponse>()
            .await?;

        if !json.ok {
            return anyhow::anyhow!("token authentication failed");
        }

        match json.result {
            Some(r) => {
                if !r.is_bot {
                    return anyhow::anyhow!("this is not a bot");
                }
            }
            None => return anyhow::anyhow!("what the heck?"),
        }

        Ok(Telegram(Arc::new(TelegramConfig {
            api_url: self.api_url,
            client: self.client,
        })))
    }
}
