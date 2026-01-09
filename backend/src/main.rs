mod struct_entity;
mod repositories;
mod db;
mod routes;

use axum::{routing::{get, post, put, delete}, Router};
use std::error::Error;
use std::env;
use std::net::SocketAddr;
use dotenv::dotenv;
use crate::db::db_config::DbInitialization;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL Failed");

     let db_init = DbInitialization::new(
        db_url.into(),
    );

    let pool = db_init.db_connect().await?;
    db_init.db_migration(&pool).await?;

    let app = Router::new()
    .route("/api/v1/book", get(|| async {"Book"}))
    .route("/api/v2/bookcoop", get(|| async {"Book"}))
    .route("/api/v3/bookcoop", get(|| async {"Book"}));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
 
    Ok(())
}
