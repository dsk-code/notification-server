use crate::{auth::channel_jwt::ChannelJwt, error::ServerError};

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct ChannelAccessToken {
    access_token: String,
    expires_in: u64,
    key_id: String,
}

pub struct AccessTokenRequest {
    jwt: ChannelJwt,
    client: Client,
}

impl AccessTokenRequest {
    pub fn new(jwt: ChannelJwt) -> Self {
        Self {
            jwt,
            client: Client::new(),
        }
    }

    pub async fn get_access_token(&self) -> Result<ChannelAccessToken, ServerError> {
        let params = [
            ("grant_type", "client_credentials"),
            (
                "client_assertion_type",
                "urn:ietf:params:oauth:client-assertion-type:jwt-bearer",
            ),
            ("client_assertion", self.jwt.token()),
        ];

        println!("Start acquiring channel access token ");
        let res = self
            .client
            .post("https://api.line.me/oauth2/v2.1/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await?
            .json::<ChannelAccessToken>()
            .await?;
        println!("Channel access token acquisition completed");

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use jsonwebtoken::{DecodingKey, EncodingKey};

    use super::*;

    use crate::Config;

    fn keys_set(config: &Config) -> (EncodingKey, DecodingKey) {
        let private_key = std::fs::read_to_string(config.private_key_path.as_str()).unwrap();
        let public_key = std::fs::read_to_string(config.public_key_path.as_str()).unwrap();

        let encoding_key = EncodingKey::from_rsa_pem(private_key.as_bytes()).unwrap();
        let decoding_key = DecodingKey::from_rsa_pem(public_key.as_bytes()).unwrap();

        (encoding_key, decoding_key)
    }

    #[tokio::test]
    async fn test_get_access_token() {
        dotenvy::dotenv().ok();
        let config = envy::from_env::<Config>().unwrap();
        let (encoding_key, _) = keys_set(&config);
        let utc_now = chrono::Utc::now();

        let jwt = ChannelJwt::create(
            config.channel_id.clone(),
            config.kid.clone(),
            utc_now,
            &encoding_key,
        )
        .unwrap();

        let req = AccessTokenRequest::new(jwt);

        let res = req.get_access_token().await;

        assert!(res.is_ok());
    }
}
