use serde::Serialize;

#[derive(Serialize)]
pub struct Book {
    pub book_id: i32,
    pub title: String,
    pub author: String,
    pub genre: Option<String>,
    pub page_count: Option<i32>,
}