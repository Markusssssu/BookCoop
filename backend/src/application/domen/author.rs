use serde::{Serialize, Deserialize};
use chrono::{NaiveDate};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Author {
    pub full_name: String,
    pub date_of_birth: NaiveDate,
    pub biography: String,
}

#[derive(Deserialize)]
pub struct NewAuthor {
    pub full_name: String,
    pub date_of_birth: NaiveDate,
    pub biography: String,
} 