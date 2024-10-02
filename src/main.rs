mod error;
mod message;

use std::sync::Arc;

use error::ServerError;
use message::LineSendMessege;

use axum::{
    response::IntoResponse,
    routing::{get, post},
    Extension, Router,
};
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Debug)]
struct State {
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

    let app = Router::new()
        .route(
            "/",
            get(|| async {
                println!("get /");
                "Hello, World!"
            }),
        )
        .route("/send", post(send))
        .layer(Extension(state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8001").await?;
    println!("* start server 0.0.0.0:8001");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn send(
    Extension(state): Extension<Arc<State>>,
    body: String,
) -> Result<impl IntoResponse, ServerError> {
    let message = LineSendMessege::new(body);
    message.send_message(state.token.clone()).await?;
    println!("send message");
    Ok(StatusCode::OK)
}
