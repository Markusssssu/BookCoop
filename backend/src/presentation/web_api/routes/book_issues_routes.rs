use axum::{Router, routing::{get, post, put, delete}};
use std::sync::Arc;

use crate::application::service::book_issues_service::BookIssueService;
use crate::presentation::web_api::handler::book_issues_handler::*;

pub fn book_issues_routes(service: Arc<BookIssueService>) -> Router {
    Router::new()
        .route("/api/issues", post(create_issue).get(get_all_issues))
        .route("/api/issues/:id",
               get(get_issue_by_id)
                   .put(update_issue)
                   .delete(delete_issue)
        )
        .with_state(service)
}
