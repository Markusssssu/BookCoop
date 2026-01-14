use axum::extract::State;
use sqlx::{PgPool, pool};


pub async fn get_dashboard(State(pool): State<PgPool>) {

}