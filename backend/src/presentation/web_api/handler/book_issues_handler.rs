use axum::{extract::{Path, State}, Json};
use std::sync::Arc;

use crate::application::services::book_issues_service::BookIssuesService;
use crate::domain::book_issues::NewBookIssue;

pub async fn create_issue(
    State(service): State<Arc<BookIssuesService>>,
    Json(payload): Json<NewBookIssue>,
) -> Result<Json<_>, (axum::http::StatusCode, String)> {
    Ok(Json(service.create_issue(payload).await.unwrap()))
}

pub async fn get_all_issues(
    State(service): State<Arc<BookIssuesService>>,
) -> Result<Json<_>, (axum::http::StatusCode, String)> {
    Ok(Json(service.get_all_issues().await.unwrap()))
}

pub async fn get_issue_by_id(
    State(service): State<Arc<BookIssuesService>>,
    Path(id): Path<i32>,
) -> Result<Json<_>, (axum::http::StatusCode, String)> {
    Ok(Json(service.get_issue_by_id(id).await.unwrap()))
}

pub async fn update_issue(
    State(service): State<Arc<BookIssuesService>>,
    Path(id): Path<i32>,
    Json(payload): Json<NewBookIssue>,
) -> Result<Json<_>, (axum::http::StatusCode, String)> {
    Ok(Json(service.update_issue(id, payload).await.unwrap()))
}

pub async fn delete_issue(
    State(service): State<Arc<BookIssuesService>>,
    Path(id): Path<i32>,
) -> Result<(), (axum::http::StatusCode, String)> {
    service.delete_issue(id).await.unwrap();
    Ok(())
}
