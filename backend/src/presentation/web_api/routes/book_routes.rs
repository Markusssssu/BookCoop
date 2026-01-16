use axum::{Router, routing::{get, post, put, delete}};
use std::sync::Arc;

use crate::presentation::web_api::handler::book_handler::*;
use crate::application::service::book_service::BookService;

pub fn book_routes(service: Arc<BookService>) -> Router {
    Router::new()
        .route("/api/books", post(create_book).get(get_all_books))
        .route("/api/books/:id",
               get(get_book_by_id)
                   .put(update_book)
                   .delete(delete_book)
        )
        .with_state(service)
}
