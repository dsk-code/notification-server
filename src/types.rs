use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EnvConfig {
    pub private_key_path: String,
    pub public_key_path: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateJWTConfig {
    pub kid: String,
    pub channel_id: String,
}
