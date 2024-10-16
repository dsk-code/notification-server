mod database;
pub mod error;
pub mod message;
pub mod router;

use crate::database::{messages::MessagesRepository, DbConnector};
use crate::message::{LineMessageKind, LineSendMessage, LineSender, ScheduledMessage};

use chrono::Local;
use error::ServerError;
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    access_token: String,
    database_url: String,
}

#[derive(Debug)]
pub struct State {
    pub pool: Arc<DbConnector>,
    pub line: Arc<LineSender>,
    pub schedule_queue: Arc<Mutex<Vec<ScheduledMessage>>>,
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
}

pub async fn init(config: Config) -> Result<State, ServerError> {
    let pool = Arc::new(database::db_init(PgPool::connect(&config.database_url).await?).await?);
    let line = Arc::new(LineSender::new(config.access_token));
    let message_repo = MessagesRepository::new(pool.clone());
    let schedule_queue = Arc::new(Mutex::new(message_repo.find_by_init().await?));
    let state = State {
        pool,
        line,
        schedule_queue,
    };

    Ok(state)
}
