mod error;
mod message;
mod router;

use crate::router::api::api;

use axum::Router;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug)]
pub struct State {
    token: String,
}

#[derive(Deserialize, Debug)]
struct Config {
    access_token: String,
}

#[tokio::main]
async fn main() -> Result<(), error::ServerError> {
    dotenvy::dotenv().ok();
    let config = envy::from_env::<Config>()?;
    let state = Arc::new(State {
        token: config.access_token,
    });
    let app = Router::new().nest("/api/v1", api(state));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8001").await?;

    println!("* start server 0.0.0.0:8001");
    axum::serve(listener, app).await?;

    Ok(())
}
