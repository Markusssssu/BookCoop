use axum::{
    routing::{get, post, put, delete},
    Router,
    http::StatusCode,
    response::IntoResponse
};

mod application;
use crate::application::
{
    domen::{
    author::Author,
    book::Book,
    dashboard::Dashboard
}, 
service::{
    delete_authors::delete_authors, delete_books::{self, delete_books}, get_authors::get_authors, get_books::get_books, get_dashboard::get_dashboard, post_authors::post_authors, post_books::{self, post_books}, put_authors::put_authors, put_books::{self, put_books}
}};

use sqlx::{postgres::PgPoolOptions};
use tower_http::cors::CorsLayer;
use dotenvy::dotenv;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok(); 
 
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DB_URL").unwrap())
        .await?;

    sqlx::migrate!("db/migrations").run(&db)
    .await?;

    println!("db connection!");

    let app = Router::new()
        .route("/api/health", post(health_check).get(health_check))
        .route("/api/books", post(post_books).get(get_books).put(put_books).delete(delete_books))
        .route("/api/authors", post(post_authors).get(get_authors).put(put_authors).delete(delete_authors))
        .route("/api/dashboard", get(get_dashboard))
        .with_state(db)
        .layer(CorsLayer::permissive());

    println!("Server listen on 8080 port");

    let listner = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listner,app)
    .await?;

    Ok(())
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}







