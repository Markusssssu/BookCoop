use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::{PgPool, Statement};
use crate::{Book, application::domen::book::NewBook};


pub async fn post_books(
    State(pool): State<PgPool>,
    Json(payload) : Json<NewBook>
) -> impl IntoResponse {
    let query = "INSERT INTO Books (title, author, genre, page_count) VALUES ($1, $2, $3, $4)";

    let result = sqlx::query(query)
    .bind(&payload.title)
    .bind(&payload.author)
    .bind(&payload.genre)
    .bind(&payload.page_count)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => (StatusCode::CREATED, "Книга успешно добавлена").into_response(),
        Err(e) => {
            eprintln!("Ошибка БД: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Ошибка при сохранении").into_response()
        }
    }
}
