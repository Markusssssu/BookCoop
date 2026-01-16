use axum::{extract::{Path, State}, Json};
use std::sync::Arc;

use crate::domain::book_issues::{BookIssue, NewBookIssue};
use crate::application::service::book_issues_service::BookIssueService;

pub async fn create_issue(
    State(service): State<Arc<BookIssueService>>,
    Json(payload): Json<NewBookIssue>,
) -> Result<Json<BookIssue>, (axum::http::StatusCode, String)> {
    let issue = service.create_issue(payload)
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(Json(issue))
}

pub async fn get_all_issues(
    State(service): State<Arc<BookIssueService>>,
) -> Result<Json<Vec<BookIssue>>, (axum::http::StatusCode, String)> {
    let issues = service.get_all_issues()
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(issues))
}

pub async fn get_issue_by_id(
    State(service): State<Arc<BookIssueService>>,
    Path(id): Path<i32>,
) -> Result<Json<BookIssue>, (axum::http::StatusCode, String)> {
    let issue = service.get_issue_by_id(id)
        .await
        .map_err(|e| (axum::http::StatusCode::NOT_FOUND, e.to_string()))?;
    Ok(Json(issue))
}

pub async fn update_issue(
    State(service): State<Arc<BookIssueService>>,
    Path(id): Path<i32>,
    Json(payload): Json<NewBookIssue>,
) -> Result<Json<BookIssue>, (axum::http::StatusCode, String)> {
    let updated = service.update_issue(id, payload)
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(Json(updated))
}

pub async fn delete_issue(
    State(service): State<Arc<BookIssueService>>,
    Path(id): Path<i32>,
) -> Result<(), (axum::http::StatusCode, String)> {
    service.delete_issue(id)
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(())
}
