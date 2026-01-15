use serde::Deserialize;
use sqlx::FromRow;

#[derive(Deserialize, sqlx::FromRow)]
pub struct Admin {
    admin_id: i32,
    full_name: String,
    keywords: String,
}

#[derive(Deserialize, sqlx::FromRow)]
pub struct NewAdmin {
    admin_id: i32,
    full_name: String,
    keywords: String,
}