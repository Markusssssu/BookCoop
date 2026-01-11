/*========use======== */
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use dotenvy::dotenv;
use std::env;
/*================== */


pub struct DBConnection {
    pool: Pool<Postgres>,
}

impl DBConnection {
    pub async fn new() -> Self {
        dotenv().ok(); 

        let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL Failed");

        let pool = PgPoolOptions::new()
        .connect(&db_url)
        .await 
        .unwrap();

        sqlx::migrate!("./migrations").run(&pool)
        .await
        .unwrap();

        Self { pool }
    }      
}

