use crate::error::ServerError;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

static KEYS: OnceLock<KeySet> = OnceLock::new();

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    iss: String,
    sub: String,
    aud: String,
    exp: i64,
    token_exp: i64,
}

struct KeySet {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

#[derive(Debug)]
pub struct Jwt {
    token: String,
}

impl Jwt {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub fn access_token(&self) -> &str {
        &self.token
    }

    pub fn create(channel_id: String, kid: String) -> Result<Self, ServerError> {
        let utc_time = Utc::now();
        let exp = utc_time + Duration::minutes(30);
        let claims = Claims {
            iss: channel_id.clone(),
            sub: channel_id.clone(),
            aud: "https://api.line.me/".to_string(),
            exp: exp.timestamp(),
            token_exp: 60 * 60 * 24 * 30,
        };

        let mut header = Header::new(jsonwebtoken::Algorithm::RS256);
        header.kid = Some(kid);
        header.typ = Some("JWT".to_string());
        let key = &KEYS.get().ok_or(ServerError::InvalidKeySet)?.encoding_key;
        let token = encode(&header, &claims, &key).map_err(|e| ServerError::InvalidEncode(e))?;

        Ok(Jwt { token })
    }

    pub fn validate(&self) -> Result<Claims, ServerError> {
        let key = &KEYS.get().ok_or(ServerError::InvalidKeySet)?.decoding_key;
        let mut validate = Validation::new(Algorithm::RS256);
        validate.set_audience(&["https://api.line.me/".to_string()]);
        let token_data = decode::<Claims>(self.access_token(), key, &validate)
            .map_err(|e| ServerError::InvalidDecode(e))?;

        Ok(token_data.claims)
    }
}

pub fn init(private_secret: &[u8], public_secret: &[u8]) -> Result<(), ServerError> {
    let key_set = KeySet {
        encoding_key: EncodingKey::from_rsa_pem(private_secret)?,
        decoding_key: DecodingKey::from_rsa_pem(public_secret)?,
    };

    KEYS.set(key_set).map_err(|_| ServerError::InvalidKeySet)
}
