use axum::{extract::{Path, State}, Json};
use std::sync::Arc;

use crate::application::services::author_service::AuthorService;
use crate::domain::author::NewAuthor;

pub async fn create_author(
    State(service): State<Arc<AuthorService>>,
    Json(payload): Json<NewAuthor>,
) -> Result<Json<_>, (axum::http::StatusCode, String)> {
    let author = service.create_author(payload)
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(author))
}

pub async fn get_all_authors(
    State(service): State<Arc<AuthorService>>,
) -> Result<Json<_>, (axum::http::StatusCode, String)> {
    Ok(Json(service.get_all_authors().await.unwrap()))
}

pub async fn get_author_by_id(
    State(service): State<Arc<AuthorService>>,
    Path(id): Path<i32>,
) -> Result<Json<_>, (axum::http::StatusCode, String)> {
    let author = service.get_author_by_id(id)
        .await
        .map_err(|e| (axum::http::StatusCode::NOT_FOUND, e.to_string()))?;

    Ok(Json(author))
}

pub async fn update_author(
    State(service): State<Arc<AuthorService>>,
    Path(id): Path<i32>,
    Json(payload): Json<NewAuthor>,
) -> Result<Json<_>, (axum::http::StatusCode, String)> {
    Ok(Json(service.update_author(id, payload).await.unwrap()))
}

pub async fn delete_author(
    State(service): State<Arc<AuthorService>>,
    Path(id): Path<i32>,
) -> Result<(), (axum::http::StatusCode, String)> {
    service.delete_author(id).await.unwrap();
    Ok(())
}
