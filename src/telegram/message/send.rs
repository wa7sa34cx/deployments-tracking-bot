//! Sending message to the Telegram bot.

use reqwest::header;
use serde_derive::{Serialize, Deserialize};

use crate::telegram::message::MessageHandler;

// https://core.telegram.org/bots/api#sendmessage
#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessage {
    pub chat_id: i64,
    pub text: String,
}

// https://core.telegram.org/bots/api#making-requests
#[derive(Debug, Deserialize)]
pub struct JsonResponse {
    pub ok: bool,
    pub result: Option<Message>,
    pub description: Option<String>,
}

// https://core.telegram.org/bots/api#message
#[derive(Debug, Deserialize)]
pub struct Message {
    pub message_id: i64,
}

impl MessageHandler {
    /// Initializes working with Telegram API, checks bot status
    pub async fn send(&self) -> anyhow::Result<()> {
        let url = format!("{}sendMessage", &self.telegram.api_url);

        let message = SendMessage {
            chat_id: self.telegram.chat_id,
            text: self.message.to_owned(),
        };

        let json = self
            .telegram
            .client
            .post(&url)
            .header(header::CONTENT_TYPE, "application/json")
            .json(&message)
            .send()
            .await?
            .json::<JsonResponse>()
            .await?;

        if !json.ok {
            return Err(anyhow::anyhow!(json
                .description
                .unwrap_or_else(|| "can't send message for unknown reason".to_string())));
        }

        log::debug!(
            "the message has been successfully sent to Telegram bot with responded id: {}",
            &json
                .result
                .map(|m| m.message_id.to_string())
                .unwrap_or_else(|| "unknown".to_string())
        );

        Ok(())
    }
}
