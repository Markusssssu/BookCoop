use axum::{extract::{Path, State}, Json};
use std::sync::Arc;
use crate::application::service::book_service::BookService;
use crate::domain::book::{Book, NewBook};

pub async fn create_book(
    State(service): State<Arc<BookService>>,
    Json(payload): Json<NewBook>,
) -> Result<Json<Book>, (axum::http::StatusCode, String)> {
    let book = service.create_book(payload)
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(Json(book))
}

pub async fn get_all_books(
    State(service): State<Arc<BookService>>,
) -> Result<Json<Vec<Book>>, (axum::http::StatusCode, String)> {
    let books = service.get_all_books()
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(books))
}

pub async fn get_book_by_id(
    State(service): State<Arc<BookService>>,
    Path(id): Path<i32>,
) -> Result<Json<Book>, (axum::http::StatusCode, String)> {
    let book = service.get_book_by_id(id)
        .await
        .map_err(|e| (axum::http::StatusCode::NOT_FOUND, e.to_string()))?;
    Ok(Json(book))
}

pub async fn update_book(
    State(service): State<Arc<BookService>>,
    Path(id): Path<i32>,
    Json(payload): Json<NewBook>,
) -> Result<Json<Book>, (axum::http::StatusCode, String)> {
    let updated = service.update_book(id, payload)
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(Json(updated))
}

pub async fn delete_book(
    State(service): State<Arc<BookService>>,
    Path(id): Path<i32>,
) -> Result<(), (axum::http::StatusCode, String)> {
    service.delete_book(id)
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(())
}
