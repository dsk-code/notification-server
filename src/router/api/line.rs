use crate::{error::ServerError, message::LineSendMessege, State};

use axum::{response::IntoResponse, routing::post, Extension, Json, Router};
use reqwest::StatusCode;
use std::sync::Arc;

pub fn router() -> Router {
    Router::new().route("/send", post(send))
}

async fn send(
    Extension(state): Extension<Arc<State>>,
    Json(payload): Json<LineSendMessege>,
) -> Result<impl IntoResponse, ServerError> {
    payload.send_message(state.token.clone()).await?;
    println!("send message");
    Ok(StatusCode::OK)
}
