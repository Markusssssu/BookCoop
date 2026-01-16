use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;

use crate::application::services::book_service::BookService;
use crate::domain::book::NewBook;

pub async fn create_book(
    State(service): State<Arc<BookService>>,
    Json(payload): Json<NewBook>,
) -> Result<Json<_>, (axum::http::StatusCode, String)> {
    let book = service.create_book(payload)
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(book))
}
