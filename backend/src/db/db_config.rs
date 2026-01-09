use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::time::Duration;

pub struct DbInitialization {
    db_url: String,
}

impl DbInitialization {
    pub fn new(db_url: String) -> Self {
        Self {
            db_url,
        }
    }

    pub async fn db_connect(&self) -> Result<Pool<Postgres>, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&self.db_url)
            .await
    }

    pub async fn db_migration(&self, pool: &sqlx::Pool<sqlx::Postgres>) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?; // 
    Ok(())
}
   
}