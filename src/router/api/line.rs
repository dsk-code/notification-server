use crate::database::messages::MessagesRepository;
use crate::model::line_webhook::{EventType, Webhook};
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
        .route("/webhook", post(webhook_handler))
        .route("/send", post(send))
        .route("/schedule", post(schedule))
}

async fn webhook_handler(Json(payload): Json<Webhook>) -> StatusCode {
    for event in payload.events {
        match event {
            EventType::Follow(follow_event) => {
                match chrono::DateTime::from_timestamp_millis(follow_event.timestamp as i64) {
                    Some(datetime) => println!("[{}] followed", datetime),
                    None => println!("Invalid time"),
                };
            }
            EventType::Unfollow(unfollow_event) => {
                match chrono::DateTime::from_timestamp_millis(unfollow_event.timestamp as i64) {
                    Some(datetime) => println!("[{}] unfollowed", datetime),
                    None => println!("Invalid time"),
                };
            }
        }
    }
    StatusCode::OK
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::line_webhook::{EventType, Webhook};

    use axum::{body, http::Request};
    use serde_json::json;
    use tower::ServiceExt;

    pub fn test_router() -> Router {
        Router::new()
            .route("/webhook", post(test_webhook_handler_fn))
            .route("/send", post(send))
            .route("/schedule", post(schedule))
    }

    async fn test_webhook_handler_fn(Json(payload): Json<Webhook>) -> StatusCode {
        for event in payload.events {
            match event {
                EventType::Follow(follow_event) => {
                    match chrono::DateTime::from_timestamp_millis(follow_event.timestamp as i64) {
                        Some(datetime) => println!("[{}] followed", datetime),
                        None => println!("Invalid time"),
                    };
                }
                EventType::Unfollow(unfollow_event) => {
                    match chrono::DateTime::from_timestamp_millis(unfollow_event.timestamp as i64) {
                        Some(datetime) => println!("[{}] unfollowed", datetime),
                        None => println!("Invalid time"),
                    };
                }
            }
        }
        StatusCode::OK
    }

    #[tokio::test]
    async fn test_webhook_handler() {
        let json_body = json!({
          "destination": "xxxxxxxxxx",
          "events": [
            {
              "type": "unfollow",
              "mode": "active",
              "timestamp": 1462629479859u64,
              "source": {
                "type": "user",
                "userId": "U4af4980629..."
              },
              "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
              "deliveryContext": {
                "isRedelivery": false
              }
            }
          ]
        })
        .to_string();

        let app = test_router();

        let req = Request::builder()
            .method("POST")
            .uri("/webhook")
            .header("Content-Type", "application/json")
            .body(body::Body::from(json_body))
            .unwrap();

        let res = app.oneshot(req).await.unwrap();

        println!("{:?}", res);
        assert_eq!(res.status(), StatusCode::OK);
    }
}
