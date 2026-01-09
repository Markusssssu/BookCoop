use chrono::NaiveDate;

pub struct Book {
    pub id: i32, 
    pub title: String,
    pub description: Option<String>,
    pub isbn: Option<String>,
    pub published: Option<NaiveDate>,
    pub admin_id: i32,
}

