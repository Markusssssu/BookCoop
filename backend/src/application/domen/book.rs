use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub genre: String,
    pub page_count: i32,
}

#[derive(Deserialize)]
pub struct NewBook {
    pub title: String,
    pub author: String,
    pub genre: String,
    pub page_count: i32,
}