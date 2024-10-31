use crate::database::DbConnector;
use crate::error::ServerError;
use crate::services::line::message::ScheduledMessage;

use chrono::{Local, NaiveDateTime};
use std::sync::Arc;

// #[derive(Debug)]
// pub struct MessagesEntity {
//     id: i32,
//     message: String,
//     send_at: NaiveDateTime,
//     created_at: NaiveDateTime,
//     updated_at: NaiveDateTime,
// }

#[derive(Debug)]
pub struct InputMessagesEntity {
    pub message: String,
    pub send_at: NaiveDateTime,
}

pub struct MessagesRepository(Arc<DbConnector>);

impl MessagesRepository {
    pub fn new(db: Arc<DbConnector>) -> Self {
        Self(db)
    }

    pub async fn add(&self, input: InputMessagesEntity) -> Result<(), ServerError> {
        let pool = self.0.get_pool();

        sqlx::query!(
            r#"
                INSERT INTO messages
                    (message, send_at)
                VALUES
                    ($1, $2)
            "#,
            input.message,
            input.send_at
        )
        .execute(&pool)
        .await
        .map_err(ServerError::InvalidDatabase)?;

        Ok(())
    }

    pub async fn find_by_init(&self) -> Result<Vec<ScheduledMessage>, ServerError> {
        let pool = self.0.get_pool();
        let now = Local::now().naive_local();
        let res = sqlx::query_as!(
            ScheduledMessage,
            r#"
                SELECT message, send_at FROM messages WHERE send_at >= $1
            "#,
            now
        )
        .fetch_all(&pool)
        .await
        .map_err(ServerError::InvalidDatabase)?;

        Ok(res)
    }
}
