use serde::{Serialize};

#[derive(Serialize, sqlx::FromRow)]
pub struct Author {
    id: u64,
    firstname: String,
    lastname: String,
    middlename: Option<String>,
    bio: Option<String>,
}