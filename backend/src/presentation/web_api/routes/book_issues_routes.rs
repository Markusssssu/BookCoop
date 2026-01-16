use axum::{Router, routing::{get, post, put, delete}};
use std::sync::Arc;

use crate::application::services::book_issues_service::BookIssuesService;
use crate::presentation::http::handlers::book_issues_handlers::*;

pub fn book_issues_routes(service: Arc<BookIssuesService>) -> Router {
    Router::new()
        .route("/issues", post(create_issue).get(get_all_issues))
        .route("/issues/:id",
               get(get_issue_by_id)
                   .put(update_issue)
                   .delete(delete_issue)
        )
        .with_state(service)
}
