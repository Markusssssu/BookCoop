use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{NaiveDate};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BookIssue {
    pub issue_id: i32,
    pub book_id: i32,
    pub author_id: i32,
    pub issue_date: NaiveDate,
    pub return_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NewBookIssue {
    pub issue_id: i32,
    pub book_id: i32,
    pub author_id: i32,
    pub issue_date: NaiveDate,
    pub return_date: Option<NaiveDate>,
}

