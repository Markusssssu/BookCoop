/*=======use========*/
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::time::sleep;
use axum::{Router, routing::{get}};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
/*================= */

/*=======use crate========*/
use crate::application::service::form_service::show_form_book;
use crate::application::service::form_service::show_form_author;
use crate::infrastructure::routes::author_routes::create_author_routes;
use crate::application::service::shutdown_service::shutdown_signal;
/*======================= */

pub struct Handler {
    listener: TcpListener,
}

impl Handler {
    pub async fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        Self { listener }
    }

    pub async fn run(self) {
                let app = Router::new()
                .route("/books", get(show_form_book).post(show_form_book))
                .route("/authors", get(show_form_author).post(show_form_author))
                .route("/slow", get(|| sleep(Duration::from_secs(5))))
                .route("/forever", get(std::future::pending::<()>))
                .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(10)),
        ));
        tracing::debug!("listening on {}", self.listener.local_addr().unwrap());

        axum::serve(self.listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
    }
}