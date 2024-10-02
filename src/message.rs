use reqwest::Client;
use serde::Serialize;

use crate::error::ServerError;

#[derive(Serialize)]
pub struct LineSendMessege {
    message: String,
}

impl LineSendMessege {
    pub fn new(message: String) -> Self {
        Self { message }
    }

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
