use axum::{extract::State, Json, response::IntoResponse, http::StatusCode};
use sqlx::{PgPool, query};

use crate::application::domen::book::Book;

pub async fn get_books(
    State(pool): State<PgPool> 
) -> impl IntoResponse {
    let query = "SELECT title, author, genre, page_count FROM Books;";
    let result = sqlx::query_as::<_, Book>(query)
        .fetch_all(&pool)
        .await;

    match result {
        Ok(books) => (StatusCode::OK, Json(books)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
