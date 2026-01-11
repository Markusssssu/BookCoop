use serde::{Serialize};

#[derive(Serialize, sqlx::FromRow)]
pub struct Book {
    id: u64,
    title: String,
    author: String,
    isbn: String, 
}