use serde::Deserialize;
use sqlx::FromRow;

#[derive(Deserialize, sqlx::FromRow)]
pub struct Admin {
    pub admin_id: i32,
    pub full_name: String,
    pub keyword: String,
}

#[derive(Deserialize, sqlx::FromRow)]
pub struct NewAdmin {
    pub admin_id: i32,
    pub full_name: String,
    pub keyword: String,
}