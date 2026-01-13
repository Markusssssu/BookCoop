/*========mod========*/
/*================== */

/*========use crate::========*/
/*================== */

/*========use========*/
use axum::{routing::{get}, Router};
use std::error::Error;
/*================== */

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> { 

    let app = Router::new()
        .route("/api/health", get(health_check));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
