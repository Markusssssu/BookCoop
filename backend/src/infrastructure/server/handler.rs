/*=======use========*/
use tokio::net::TcpListener;
use axum::{Router, routing::{get}};
/*================= */

/*=======use crate========*/
use crate::infrastructure::log::logger_init::tracing_initialization;
use crate::application::service::form_service::show_form;
use crate::infrastructure::routes::author_routes::create_author_routes;
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
                .route("/auth", get(show_form))
                .merge(create_author_routes());
        tracing::debug!("listening on {}", self.listener.local_addr().unwrap());

        axum::serve(self.listener, app).await.unwrap();
    }
}