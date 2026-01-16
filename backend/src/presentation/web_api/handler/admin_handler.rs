use axum::{extract::{Path, State}, Extension, Json};
use std::sync::Arc;
use crate::application::service::admin_service::AdminService;
use crate::domain::admin::{Admin, NewAdmin};


pub async fn get_all_admins(
    Extension(service): Extension<Arc<AdminService>>,
) -> Result<Json<Vec<Admin>>, (axum::http::StatusCode, String)> {
    let admins = service.list_admins().await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(admins))
}

// остальные хендлеры точно так же, через Extension


pub async fn get_admin_by_id(
    Extension(service): Extension<Arc<AdminService>>,
    Path(id): Path<i32>,
) -> Result<Json<Admin>, (axum::http::StatusCode, String)> {
    let admin = service.get_admin_by_id(id)
        .await
        .map_err(|e| (axum::http::StatusCode::NOT_FOUND, e.to_string()))?;
    Ok(Json(admin))
}

pub async fn create_admin(
    Extension(service): Extension<Arc<AdminService>>,
    Json(payload): Json<NewAdmin>,
) -> Result<Json<Admin>, (axum::http::StatusCode, String)> {
    let admin = service.create_admin(payload)
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(Json(admin))
}

pub async fn update_admin(
    Extension(service): Extension<Arc<AdminService>>,
    Path(id): Path<i32>,
    Json(payload): Json<NewAdmin>,
) -> Result<Json<Admin>, (axum::http::StatusCode, String)> {
    let updated = service.update_admin(id, payload)
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(Json(updated))
}

pub async fn delete_admin(
    Extension(service): Extension<Arc<AdminService>>,
    Path(id): Path<i32>,
) -> Result<(), (axum::http::StatusCode, String)> {
    service.delete_admin(id)
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(())
}
