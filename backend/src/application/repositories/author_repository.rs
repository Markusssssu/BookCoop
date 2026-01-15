use async_trait::async_trait;
use crate::domain::author::{Author, NewAuthor};

#[async_trait]
pub trait AuthorRepository {

    /*=======Base CRUD=======*/
    async fn insert(&self, book: NewAuthor) -> Result<Author, anyhow::Error>;
    async fn find_all(&self) -> Result<Vec<Author>, anyhow::Error>;
    async fn find_by_id(&self, id: i32) -> Result<Option<Author>, anyhow::Error>;
    async fn find_by_full_name(&self, title: &str) -> Result<Vec<Author>, anyhow::Error>;
    async fn update(&self, id: i32, book_issue: NewAuthor) -> Result<Author, anyhow::Error>;
    async fn delete(&self, id: i32) -> Result<(), anyhow::Error>;

    /*=======================*/

}