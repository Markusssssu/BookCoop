use async_trait::async_trait;
use crate::domain::book_issues::{BookIssue, NewBookIssue};

#[async_trait]
pub trait BookIssuesRepository {

    /*=======Base CRUD=======*/
    async fn insert(&self, book: NewBookIssue) -> Result<BookIssue, anyhow::Error>;
    async fn find_all(&self) -> Result<Vec<BookIssue>, anyhow::Error>;
    async fn find_by_id(&self, id: i32) -> Result<Option<BookIssue>, anyhow::Error>;
    async fn find_by_title(&self, title: &str) -> Result<Vec<BookIssue>, anyhow::Error>;
    async fn update(&self, id: i32, book_issue: NewBookIssue) -> Result<BookIssue, anyhow::Error>;
    async fn delete(&self, id: i32) -> Result<(), anyhow::Error>;

    /*=======================*/
}