pub mod auth;
mod database;
pub mod error;
pub mod message;
pub mod model;
pub mod router;

use crate::auth::KEYS;
use crate::auth::{channel_access_token::ChannelAccessToken, channel_jwt::ChannelJwt};
use crate::database::{messages::MessagesRepository, DbConnector};
use crate::message::{LineMessageKind, LineSendMessage, LineSender, ScheduledMessage};

use auth::channel_access_token::AccessTokenRequest;
use chrono::{Local, Utc};
use error::ServerError;
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{interval, sleep, Duration};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    access_token: String,
    database_url: String,
    private_key_path: String,
    public_key_path: String,
    kid: String,
    channel_id: String,
}

#[derive(Debug)]
pub struct State {
    pub pool: Arc<DbConnector>,
    pub line: Arc<LineSender>,
    pub schedule_queue: Arc<Mutex<Vec<ScheduledMessage>>>,
    pub channel_access_token: Arc<Mutex<ChannelAccessToken>>,
}

impl State {
    pub async fn polling_task(self: Arc<Self>) {
        let mut interval = interval(Duration::from_secs(1));
        loop {
            interval.tick().await;

            let now = Local::now();
            println!("current time: {}", now);
            let mut queue = self.schedule_queue.lock().await;
            let (to_send, remaining): (Vec<_>, Vec<_>) = queue
                .drain(..)
                .partition(|msg| msg.send_at <= now.naive_local());
            println!("number of messages remaining: {}", remaining.len());
            queue.extend(remaining);
            drop(queue);

            for msg in to_send {
                let message = LineSendMessage {
                    message: msg.message.clone(),
                };
                if let Err(e) = self.line.send(LineMessageKind::Version1(message)).await {
                    eprintln!("{}", e);
                }
            }
        }
    }

    pub async fn get_access_token_scheduled_task(&self, config: Config) -> Result<(), ServerError> {
        sleep(Duration::from_secs(30 * 24 * 60 * 60)).await;

        let mut interval = interval(Duration::from_secs(30 * 24 * 60 * 60));
        loop {
            interval.tick().await;

            let channel_access_token = set_channel_access_token(config.clone()).await?;
            let mut token_guard = self.channel_access_token.lock().await;
            *token_guard = channel_access_token;
        }
    }
}

pub async fn init(config: Config) -> Result<State, ServerError> {
    let private_key = std::fs::read_to_string(config.private_key_path.as_str())?;
    let public_key = std::fs::read_to_string(config.public_key_path.as_str())?;

    auth::auth_init(private_key.as_bytes(), public_key.as_bytes())?;

    let channel_access_token =
        Arc::new(Mutex::new(set_channel_access_token(config.clone()).await?));

    let pool = Arc::new(database::db_init(PgPool::connect(&config.database_url).await?).await?);

    let line = Arc::new(LineSender::new(config.access_token));

    let message_repo = MessagesRepository::new(pool.clone());
    let schedule_queue = Arc::new(Mutex::new(message_repo.find_by_init().await?));

    let state = State {
        pool,
        line,
        schedule_queue,
        channel_access_token,
    };

    Ok(state)
}

async fn set_channel_access_token(config: Config) -> Result<ChannelAccessToken, ServerError> {
    let utc_now = Utc::now();
    let encoding_key = &KEYS.get().ok_or(ServerError::InvalidKeySet)?.encoding_key;

    let jwt = ChannelJwt::create(
        config.channel_id.clone(),
        config.kid.clone(),
        utc_now,
        encoding_key,
    )?;

    let channel_token_req = AccessTokenRequest::new(jwt);
    let token = channel_token_req.get_access_token().await?;

    Ok(token)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test_set_channel_access_token() {

//     }

// }
