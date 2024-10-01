use std::path::Path;

use notification_server::auth::{init, Jwt};
use notification_server::error;
use notification_server::types::{EnvConfig, CreateJWTConfig};

use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() -> Result<(), error::ServerError> {
    dotenvy::dotenv().ok();
    let env_config = envy::from_env::<EnvConfig>()?;
    let create_jwt_config = envy::from_env::<CreateJWTConfig>()?;
    let private_key_path = Path::new(env_config.private_key_path.as_str());
    let public_key_path = Path::new(env_config.public_key_path.as_str());
    let private_key_pem = std::fs::read_to_string(private_key_path)?;
    let public_key_pem = std::fs::read_to_string(public_key_path)?;
    let private_secret = private_key_pem.as_bytes();
    let public_secret = public_key_pem.as_bytes();
    init(private_secret, public_secret)?;
    let jwt = Jwt::create(create_jwt_config.channel_id, create_jwt_config.kid)?;
    jwt.validate()?;
    
    let app = Router::new()
        .route(
            "/",
            get(|| async {
                println!("get /");
                "Hello, World!"
            }).post(post_foo),
        )
        .route("/reply", post(post_reply));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8001").await?;
    println!("* start server 0.0.0.0:8001");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn post_reply() {}
async fn post_foo() {}
