use notification_server::{self as server, State};

use server::error::ServerError;
use server::message::LineSender;
use server::router::api::api;

use axum::Router;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize, Debug)]
struct Config {
    access_token: String,
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    dotenvy::dotenv().ok();
    let config = envy::from_env::<Config>()?;
    let line = Arc::new(LineSender::new(config.access_token));
    let schedule_queue = Arc::new(Mutex::new(Vec::new()));
    let state = Arc::new(State {
        line,
        schedule_queue,
    });
    let cloned_state: Arc<State> = Arc::clone(&state);
    tokio::spawn(async move {
        cloned_state.polling_task().await;
    });
    let app = Router::new().nest("/api/v1", api(state));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8001").await?;

    println!("* start server 0.0.0.0:8001");
    axum::serve(listener, app).await?;

    Ok(())
}
