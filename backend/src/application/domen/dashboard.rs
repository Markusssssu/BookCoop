use serde::Serialize;

#[derive(Serialize)]
pub struct Dashboard {
    pub books: i64,
    pub authors: i64,
    pub today: i64,
    pub cpu: f32
}