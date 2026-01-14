use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::PgPool; 
use crate::application::domen::author::NewAuthor; 

pub async fn post_authors(
    State(pool): State<PgPool>,
    Json(payload): Json<NewAuthor>
) -> impl IntoResponse {

    let query = "INSERT INTO Authors (full_name, date_of_birth, biography) VALUES ($1, $2, $3)";

    let result = sqlx::query(query)
        .bind(&payload.full_name)
        .bind(payload.date_of_birth) // Передаем копию даты (NaiveDate реализует Copy)
        .bind(&payload.biography)    // Если это String
        .execute(&pool)
        .await;

    match result {
        Ok(_) => (StatusCode::CREATED, "Автор успешно добавлен").into_response(),
        Err(e) => {
            eprintln!("Database Error: {:?}", e); 
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Ошибка БД: {}", e)).into_response()
        }
    }
}
