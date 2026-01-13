use serde::Serialize;

#[derive(Serialize)]
pub struct Author {
    pub author_id: i32,
    pub full_name: String,
    pub date_of_birth: Option<String>,
    pub biography: Option<String>,
}