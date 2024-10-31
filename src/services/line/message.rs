use crate::error::ServerError;
use crate::model::line_webhook::Emoji;

use chrono::NaiveDateTime;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LineSendMessage {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduledMessage {
    pub message: String,
    pub send_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMessage {
    #[serde(rename = "replyToken")]
    pub reply_token: String,
    pub messages: Vec<Message>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "type")]
    pub message_type: String,
    pub text: String,
    pub emojis: Option<Vec<Emoji>>
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Emoji {
//     pub index: usize,
//     #[serde(rename = "productId")]
//     pub product_id: String,
//     #[serde(rename = "emojiId")]
//     pub emoji_id: String,
// }

pub enum LineMessageKind {
    Version1(LineSendMessage),
    Version2(ResponseMessage),
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
            // 旧型なので廃止するか検討
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
            },
            LineMessageKind::Version2(message) => {
                self.client
                    .post("https://api.line.me/v2/bot/message/reply")
                    .bearer_auth(self.access_token.clone())
                    .header("Content-Type", "application/json")
                    .json(&message)
                    .send()
                    .await?;
                println!("sent a message");
                Ok(())
            },

        }
    }

    // pub async fn send_scheduled() -> Result<(), ServerError> {

    // }
}
