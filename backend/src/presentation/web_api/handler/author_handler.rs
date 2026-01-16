use axum::{extract::{Path, State}, Json};
use std::sync::Arc;

use crate::application::service::author_service::AuthorService;
use crate::domain::author::{Author, NewAuthor};

pub async fn create_author(
    State(service): State<Arc<AuthorService>>,
    Json(payload): Json<NewAuthor>,
) -> Result<Json<Author>, (axum::http::StatusCode, String)> {
    let author = service
        .create_author(payload)
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(author))
}

pub async fn get_all_authors(
    State(service): State<Arc<AuthorService>>,
) -> Result<Json<Vec<Author>>, (axum::http::StatusCode, String)> {
    let authors = service
        .get_all_authors()
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(authors))
}

pub async fn get_author_by_id(
    State(service): State<Arc<AuthorService>>,
    Path(id): Path<i32>,
) -> Result<Json<Author>, (axum::http::StatusCode, String)> {
    let author = service
        .get_author_by_id(id)
        .await
        .map_err(|e| (axum::http::StatusCode::NOT_FOUND, e.to_string()))?;

    Ok(Json(author))
}

pub async fn update_author(
    State(service): State<Arc<AuthorService>>,
    Path(id): Path<i32>,
    Json(payload): Json<NewAuthor>,
) -> Result<Json<Author>, (axum::http::StatusCode, String)> {
    let updated_author = service
        .update_author(id, payload)
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(updated_author))
}

pub async fn delete_author(
    State(service): State<Arc<AuthorService>>,
    Path(id): Path<i32>,
) -> Result<(), (axum::http::StatusCode, String)> {
    service
        .delete_author(id)
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(())
}
