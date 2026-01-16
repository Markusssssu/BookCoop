mod application;
mod domain;
mod infrastructure;
mod presentation;

use axum::{routing::{get}, Router};

use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;
use std::error::Error;
use http::StatusCode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    //DB_URL=postgresql://postgres:Gjitkyf[eq102@localhost:5432/bookcoop

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DB_URL").unwrap())
        .await?;

    sqlx::migrate!("db/migrations").run(&db)
    .await?;

    log::info!("database bookcoop Connection");

    log::info!("starting HTTP server at http://localhost:8080");

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/api/health", get(|| async { StatusCode::OK }));

    let listner = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listner,app.into_make_service()).await?;

    Ok(())
}







