pub mod channel_access_token;
pub mod channel_jwt;

use crate::error::ServerError;

use jsonwebtoken::{DecodingKey, EncodingKey};
use std::sync::OnceLock;

pub static KEYS: OnceLock<KeySet> = OnceLock::new();

pub struct KeySet {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
}

pub fn auth_init(private_secret: &[u8], public_secret: &[u8]) -> Result<(), ServerError> {
    let key_set = KeySet {
        encoding_key: EncodingKey::from_rsa_pem(private_secret)?,
        decoding_key: DecodingKey::from_rsa_pem(public_secret)?,
    };

    KEYS.set(key_set).map_err(|_| ServerError::InvalidKeySet)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Config;

    #[test]
    fn test_auth_init() {
        dotenvy::dotenv().ok();
        let config = envy::from_env::<Config>().unwrap();
        let private_key = std::fs::read_to_string(config.private_key_path.as_str()).unwrap();
        let public_key = std::fs::read_to_string(config.public_key_path.as_str()).unwrap();

        let result = auth_init(private_key.as_bytes(), public_key.as_bytes());

        assert!(result.is_ok());
    }
}
