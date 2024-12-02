pub mod messages;

use crate::error::{self, ServerError};
use sqlx::PgPool;

#[derive(Debug)]
pub struct DbConnector {
    pool: PgPool,
}

impl DbConnector {
    pub fn new(pool: PgPool) -> Self {
        DbConnector { pool }
    }

    pub fn get_pool(&self) -> PgPool {
        self.pool.clone()
    }

    pub async fn migration(&self) -> Result<(), ServerError> {
        println!("Start migration");
        sqlx::migrate!("./migrations/")
            .run(&self.pool)
            .await
            .map_err(error::ServerError::InvalidDatabaseMigration)?;
        println!("Migration completed");

        Ok(())
    }
}

pub async fn db_init(pool: PgPool) -> Result<DbConnector, ServerError> {
    let db = DbConnector::new(pool);
    db.migration().await?;

    Ok(db)
}
