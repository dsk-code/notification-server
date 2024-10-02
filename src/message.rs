use crate::error::ServerError;

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LineSendMessege {
    message: String,
}

impl LineSendMessege {
    pub async fn send_message(&self, token: String) -> Result<(), ServerError> {
        let encode_message = serde_urlencoded::to_string(self)?;
        let client = Client::new();
        client
            .post("https://notify-api.line.me/api/notify")
            .bearer_auth(token)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(encode_message)
            .send()
            .await?;

        Ok(())
    }
}
