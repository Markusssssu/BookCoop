use axum::{Router, routing::{get, post, put, delete}};
use std::sync::Arc;

use crate::application::service::author_service::AuthorService;
use crate::presentation::web_api::handler::author_handler::*;

pub fn author_routes(service: Arc<AuthorService>) -> Router {
    Router::new()
        .route("/authors", post(create_author).get(get_all_authors))
        .route("/authors/:id",
               get(get_author_by_id)
                   .put(update_author)
                   .delete(delete_author)
        )
        .with_state(service)
}
