/*=======use======= */
use axum::{Router, routing::{Route, get, post}};

use crate::application::service::form_service::show_form;
/*================= */

pub fn create_author_routes() -> Router {
    Router::new()
        .route("/book", get(show_form))
        .route("/book", post(show_form))
}