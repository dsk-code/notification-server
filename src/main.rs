use notification_server as server;

use crate::server::error::ServerError;
use crate::server::router::api::api;
use crate::server::{Config, State};

use axum::Router;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    dotenvy::dotenv().ok();
    let config = envy::from_env::<Config>()?;

    let state = Arc::new(server::init(config.clone()).await?);

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
