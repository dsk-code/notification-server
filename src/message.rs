use crate::error::ServerError;

use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LineSendMessage {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduledMessage {
    pub message: String,
    pub send_at: DateTime<Utc>,
}

pub enum LineMessageKind {
    Version1(LineSendMessage),
}

#[derive(Debug)]
pub struct LineSender {
    client: Client,
    access_token: String,
}

impl LineSender {
    pub fn new(access_token: String) -> Self {
        Self {
            client: Client::new(),
            access_token,
        }
    }

    pub async fn send(&self, message: LineMessageKind) -> Result<(), ServerError> {
        match message {
            LineMessageKind::Version1(message) => {
                let encode_message = serde_urlencoded::to_string(message)?;
                self.client
                    .post("https://notify-api.line.me/api/notify")
                    .bearer_auth(self.access_token.clone())
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body(encode_message)
                    .send()
                    .await?;
                println!("sent a message");
                Ok(())
            }
        }
    }

    // pub async fn send_scheduled() -> Result<(), ServerError> {

    // }
}
