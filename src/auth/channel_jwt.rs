// use crate::auth::KEYS;
use crate::error::ServerError;

use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Claims {
    iss: String,
    sub: String,
    aud: String,
    exp: i64,
    token_exp: i64,
}

#[derive(Debug)]
pub struct ChannelJwt {
    token: String,
}

impl ChannelJwt {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn create(
        channel_id: String,
        kid: String,
        utc_now: DateTime<Utc>,
        key: &EncodingKey,
    ) -> Result<Self, ServerError> {
        let exp = utc_now + Duration::minutes(30);
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

        // let key = &KEYS.get().ok_or(ServerError::InvalidKeySet)?.encoding_key;

        let token = encode(&header, &claims, &key).map_err(|e| ServerError::InvalidEncode(e))?;

        Ok(ChannelJwt { token })
    }

    pub fn validate(&self, key: DecodingKey) -> Result<Claims, ServerError> {
        // let key = &KEYS.get().ok_or(ServerError::InvalidKeySet)?.decoding_key;

        let mut validate = Validation::new(Algorithm::RS256);
        validate.set_audience(&["https://api.line.me/".to_string()]);

        let token_data = decode::<Claims>(self.token(), &key, &validate)
            .map_err(|e| ServerError::InvalidDecode(e))?;

        Ok(token_data.claims)
    }
}


// チャネルアクセストークンが2024-11-25まで発行できないので待機
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::Config;


//     fn keys_set(config: &Config) -> (EncodingKey, DecodingKey) {
//         let private_key = std::fs::read_to_string(config.private_key_path.as_str()).unwrap();
//         let public_key = std::fs::read_to_string(config.public_key_path.as_str()).unwrap();

//         let encoding_key = EncodingKey::from_rsa_pem(private_key.as_bytes()).unwrap();
//         let decoding_key = DecodingKey::from_rsa_pem(public_key.as_bytes()).unwrap();

//         (encoding_key, decoding_key)
//     }

//     #[test]
//     fn test_channel_jwt_new() {
//         let data = "jkdoajjejmbmkljkfkdl".to_string();
//         let jwt = ChannelJwt::new(data.clone());

//         assert_eq!(data, jwt.token);
//     }

    // #[test]
    // fn test_channel_jwt_validate() {
    //     dotenvy::dotenv().ok();
    //     let config = envy::from_env::<Config>().unwrap();
    //     let (encoding_key, decoding_key) = keys_set(&config);
    //     let utc_now = chrono::Utc::now();

    //     let jwt = ChannelJwt::create(
    //         config.channel_id.clone(),
    //         config.kid.clone(),
    //         utc_now,
    //         &encoding_key,
    //     )
    //     .unwrap();

    //     let claims = jwt.validate(decoding_key).unwrap();

    //     assert_eq!(config.channel_id, claims.iss);
    //     assert_eq!(config.channel_id, claims.sub);
    // }
// }
