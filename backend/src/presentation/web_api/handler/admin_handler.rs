use axum::{extract::{Path, State}, Json};
use std::sync::Arc;

use crate::application::services::admin_service::AdminService;
use crate::domain::admin::NewAdmin;

pub async fn create_admin(
    State(service): State<Arc<AdminService>>,
    Json(payload): Json<NewAdmin>,
) -> Result<Json<_>, (axum::http::StatusCode, String)> {
    Ok(Json(service.create_admin(payload).await.unwrap()))
}

pub async fn get_all_admins(
    State(service): State<Arc<AdminService>>,
) -> Result<Json<_>, (axum::http::StatusCode, String)> {
    Ok(Json(service.get_all_admins().await.unwrap()))
}

pub async fn get_admin_by_id(
    State(service): State<Arc<AdminService>>,
    Path(id): Path<i32>,
) -> Result<Json<_>, (axum::http::StatusCode, String)> {
    Ok(Json(service.get_admin_by_id(id).await.unwrap()))
}

pub async fn update_admin(
    State(service): State<Arc<AdminService>>,
    Path(id): Path<i32>,
    Json(payload): Json<NewAdmin>,
) -> Result<Json<_>, (axum::http::StatusCode, String)> {
    Ok(Json(service.update_admin(id, payload).await.unwrap()))
}

pub async fn delete_admin(
    State(service): State<Arc<AdminService>>,
    Path(id): Path<i32>,
) -> Result<(), (axum::http::StatusCode, String)> {
    service.delete_admin(id).await.unwrap();
    Ok(())
}
