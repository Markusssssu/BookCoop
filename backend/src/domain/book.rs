use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Book {
    pub book_id: i32,
    pub title: String,
    pub author: String,
    pub genre: String,
    pub page_count: i32,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct NewBook {
    pub title: String,
    pub author: String,
    pub genre: String,
    pub page_count: i32,
}





