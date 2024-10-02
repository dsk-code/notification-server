use crate::{error::ServerError, message::LineSendMessege, State};

use axum::{response::IntoResponse, routing::post, Extension, Router};
use reqwest::StatusCode;
use std::sync::Arc;

pub fn router() -> Router {
    Router::new().route("/send", post(send))
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
