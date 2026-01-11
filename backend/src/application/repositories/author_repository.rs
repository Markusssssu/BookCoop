/*========use======= */
use crate::domain::author::Author;
use std::io::Error;
/*================== */

pub trait AuthorRepository {
    async fn insert(&self, author: Author) -> Result<Author, Error>;
    async fn get(&self, id: i64) -> Result<Option<Author>, Error>;
    async fn list(&self) -> Result<Vec<Author>, Error>;
    async fn update(&self, auhtor: Author) -> Result<Author, Error>;
    async fn delete(&self, id: i64) -> Result<(), Error>;
}