/*========mod========*/
pub mod application;
/*================== */

/*========use crate::========*/
use crate::application::service::{
    get_books::get_books, 
    get_authors::get_authors, 
    get_dashboard::get_dashboard
};
/*================== */

/*========use========*/
use axum::{routing::{get}, Router};
use std::env;
use dotenvy::dotenv;
use std::error::Error;
use tower_http::cors::CorsLayer;
/*================== */

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> { 
    dotenv().ok();

    let db_url = env::var("DB_URL")?;

    let pool = sqlx::PgPool::connect(&db_url).await?;

    sqlx::migrate!("db/migrations")
    .run(&pool)
    .await?;

    println!("Database migrations applied successfully.");

    let app = Router::new()

    let app = Router::new()
        .route("/api/books", get(get_books))
        .route("/api/authors", get(get_authors))
        .route("/api/dashboard", get(get_dashboard))
        .route("/api/health", get(health_check))
        .with_state(pool)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    
    axum::serve(listener, app).await?;
    println!("Server running on http://0.0.0.0:8080");

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
