use async_trait::async_trait;
use crate::domain::book::{Book, NewBook};

#[async_trait]
pub trait BookRepository {

    /*=======Base CRUD=======*/
    async fn insert(&self, book: NewBook) -> Result<Book, anyhow::Error>;
    async fn find_all(&self) -> Result<Vec<Book>, anyhow::Error>;
    async fn find_by_id(&self, id: i32) -> Result<Option<Book>, anyhow::Error>;
    async fn find_by_title(&self, title: &str) -> Result<Vec<Book>, anyhow::Error>;
    async fn update(&self, id: i32, book: NewBook) -> Result<Book, anyhow::Error>;
    async fn delete(&self, id: i32) -> Result<(), anyhow::Error>;

    /*=======================*/
}