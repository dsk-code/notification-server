use crate::database::messages::MessagesRepository;
use crate::model::line_webhook::{EventType, MessageType, Webhook};
use crate::{
    database::messages::InputMessagesEntity,
    error::ServerError,
    State,
};
use crate::services::line::message::{LineMessageKind, LineSendMessage, ScheduledMessage, ResponseMessage, Message};

use axum::{response::IntoResponse, routing::post, Extension, Json, Router};
use reqwest::StatusCode;
use std::sync::Arc;

pub fn router() -> Router {
    Router::new()
        .route("/webhook", post(webhook_handler))
        .route("/send", post(send))
        .route("/schedule", post(schedule))
}

async fn webhook_handler(Extension(state): Extension<Arc<State>>, Json(payload): Json<Webhook>) -> StatusCode {
    for event in payload.events {
        match event {
            EventType::Message(message_event) => {
                match chrono::DateTime::from_timestamp_millis(message_event.timestamp) {
                    Some(datetime) => {
                        match &message_event.message {
                            MessageType::Text(text_message) => {
                                // 文字列にLINEの絵文字が入っているとメッセージが返信されないので、改善途中
                                // "こんにちは$"のように絵文字のところに'$'を追加できるように実装したい
                                // 
                                // match text_message.emojis.clone() {
                                //     Some(emojis) => {
                                //         let indexes: Vec<usize> = emojis.iter().map(|emoji| emoji.index).collect();
                                //         let s = text_message.text.clone();
                                //     }
                                // }
                                
                                let mut messages = Vec::new();
                                let message = Message {
                                    message_type: "text".to_string(),
                                    text: text_message.text.clone(),
                                    emojis: text_message.emojis.clone(),
                                };
                                messages.push(message);
                                let response_message = ResponseMessage {
                                    reply_token: message_event.reply_token,
                                    messages,
                                };
                                let _ = state.line.send(LineMessageKind::Version2(response_message)).await;
                                println!("[{}] I got a message", datetime)
                            }
                        }},
                    None => println!("Invalid time"),
                };
            },
            EventType::Follow(follow_event) => {
                match chrono::DateTime::from_timestamp_millis(follow_event.timestamp) {
                    Some(datetime) => println!("[{}] followed", datetime),
                    None => println!("Invalid time"),
                };
            },
            EventType::Unfollow(unfollow_event) => {
                match chrono::DateTime::from_timestamp_millis(unfollow_event.timestamp) {
                    Some(datetime) => println!("[{}] unfollowed", datetime),
                    None => println!("Invalid time"),
                };
            },
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
            // .route("/send", post(send))
            // .route("/schedule", post(schedule))
    }

    async fn test_webhook_handler_fn(Json(payload): Json<Webhook>) -> StatusCode {
        for event in payload.events {
            match event {
                EventType::Message(message_event) => {
                    match chrono::DateTime::from_timestamp_millis(message_event.timestamp) {
                        Some(datetime) => {
                            match message_event.message {
                                MessageType::Text(text_message) => println!("[{}] {}", datetime, text_message.text)
                            }},
                        None => println!("Invalid time"),
                    };
                },
                EventType::Follow(follow_event) => {
                    match chrono::DateTime::from_timestamp_millis(follow_event.timestamp) {
                        Some(datetime) => println!("[{}] followed", datetime),
                        None => println!("Invalid time"),
                    };
                }
                EventType::Unfollow(unfollow_event) => {
                    match chrono::DateTime::from_timestamp_millis(unfollow_event.timestamp) {
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
                "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
                "type": "message",
                "mode": "active",
                "timestamp": 1462629479859i64,
                "source": {
                  "type": "group",
                  "groupId": "Ca56f94637c...",
                  "userId": "U4af4980629..."
                },
                "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
                "deliveryContext": {
                  "isRedelivery": false
                },
                "message": {
                  "id": "444573844083572737",
                  "type": "text",
                  "quoteToken": "q3Plxr4AgKd...",
                  "text": "@All @example Good Morning!! (love)",
                  "emojis": [
                    {
                      "index": 29,
                      "length": 6,
                      "productId": "5ac1bfd5040ab15980c9b435",
                      "emojiId": "001"
                    }
                  ],
                  "mention": {
                    "mentionees": [
                      {
                        "index": 0,
                        "length": 4,
                        "type": "all"
                      },
                      {
                        "index": 5,
                        "length": 8,
                        "userId": "U49585cd0d5...",
                        "type": "user"
                      }
                    ]
                  }
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
