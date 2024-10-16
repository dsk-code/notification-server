use crate::database::messages::MessagesRepository;
use crate::{
    database::messages::InputMessagesEntity,
    error::ServerError,
    message::{LineMessageKind, LineSendMessage, ScheduledMessage},
    State,
};

use axum::{response::IntoResponse, routing::post, Extension, Json, Router};
use reqwest::StatusCode;
use std::sync::Arc;

pub fn router() -> Router {
    Router::new()
        .route("/send", post(send))
        .route("/schedule", post(schedule))
}

async fn send(
    Extension(state): Extension<Arc<State>>,
    Json(payload): Json<LineSendMessage>,
) -> Result<impl IntoResponse, ServerError> {
    state.line.send(LineMessageKind::Version1(payload)).await?;
    println!("send message");
    Ok(StatusCode::OK)
}

async fn schedule(
    Extension(state): Extension<Arc<State>>,
    Json(payload): Json<ScheduledMessage>,
) -> Result<impl IntoResponse, ServerError> {
    let db = MessagesRepository::new(state.pool.clone());
    println!("Start adding to database");
    db.add(InputMessagesEntity {
        message: payload.message.clone(),
        send_at: payload.send_at,
    })
    .await?;
    println!("Added to database completed");
    let mut queue = state.schedule_queue.lock().await;
    queue.push(payload);
    drop(queue);
    println!("schedule message");
    Ok(StatusCode::OK)
}
