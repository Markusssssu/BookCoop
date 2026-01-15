use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Dashboard {
    pub books: i64,
    pub authors: i64,
    pub today: i64,
    pub cpu: f32
}