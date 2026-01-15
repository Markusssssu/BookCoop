use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sqlx::{PgPool, pool};
use crate::application::domen::author::Author;
use crate::application::domen::book::Book;
use crate::application::domen::dashboard::Dashboard;

pub async fn get_dashboard(State(pool): State<PgPool>) -> impl IntoResponse {
    let books_count = "SELECT COUNT(*) FROM Books;";
    let authors_count = "SELECT COUNT(*) FROM Authors;";
    let book_issues_count = "SELECT COUNT(*) FROM Book_Issues;";

    let result_books_count = sqlx::query_as::<_, Book>(books_count)
        .fetch_all(&pool)
        .await;
    
    let result_authors_count = sqlx::query_as::<_, Author>(authors_count)
        .fetch_all(&pool)
        .await;

    let result_book_issues_count = sqlx::query_as::<_, Dashboard>(book_issues_count)
        .fetch_all(&pool)
        .await;
    
    match result_books_count {
        Ok(books_count) => (StatusCode::OK, Json(books_count)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };
    
    match result_authors_count {
        Ok(authors_count) => (StatusCode::OK, Json(authors_count)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };
    
    match result_book_issues_count {
        Ok(books_issues_count) => (StatusCode::OK, Json(books_issues_count)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}