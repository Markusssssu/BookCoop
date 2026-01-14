use axum::{extract::State, response::IntoResponse};
use sqlx::{PgPool, pool, query};
use crate::application::domen::author::Author;

pub async fn get_authors(
    State(pool) : State<PgPool>
) -> impl IntoResponse {
    let query = "SELECT full_name, date_of_birth, biography FROM Authors;";
    let result = sqlx::query_as::<_, Author>(query)
        .fetch_all(&pool)
        .await;

    match result {
        Ok(authors) => (axum::http::StatusCode::OK, axum::Json(authors)).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
