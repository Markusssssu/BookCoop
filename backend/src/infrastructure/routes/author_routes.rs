/*=======use======= */
use axum::{Router, routing::{Route, get, post}};

use crate::application::service::form_service::show_form_book;
/*================= */

pub fn create_author_routes() -> Router {
    Router::new()
        .route("/book", get(show_form_book))
        .route("/book", post(show_form_book))
}