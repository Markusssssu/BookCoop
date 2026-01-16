use axum::Router;
use std::sync::Arc;

use crate::application::service::{
    book_service::BookService,
    author_service::AuthorService,
    admin_service::AdminService,
    book_issues_service::BookIssueService,
};

pub mod book_routes;
pub mod author_routes;
pub mod admin_routes;
pub mod book_issues_routes;

pub fn app_routes(
    book: Arc<BookService>,
    author: Arc<AuthorService>,
    admin: Arc<AdminService>,
    issues: Arc<BookIssueService>,
) -> Router {
    Router::new()
        .merge(book_routes::book_routes(book))
        .merge(author_routes::author_routes(author))
        // .merge(admin_routes::admin_routes(admin))
        .merge(book_issues_routes::book_issues_routes(issues))
}
