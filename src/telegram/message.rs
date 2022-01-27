//! Creating MessageHandler.

use crate::telegram::Telegram;

pub mod send;

/// Message handler
#[derive(Debug)]
pub struct MessageHandler {
    pub telegram: Telegram,
    pub message: String,
}

impl Telegram {
    /// Creates deployments endpoint handler
    pub fn message(&self, message: impl Into<String>) -> MessageHandler {
        MessageHandler {
            telegram: self.clone(),
            message: message.into(),
        }
    }
}
