/*========use======= */
use crate::domain::book::Book;
use std::io::Error;
/*================== */

pub trait BookRepository {
    async fn insert(&self, book: Book) -> Result<Book, Error>;
    async fn get(&self, id: i64) -> Result<Option<Book>, Error >;
    async fn list(&self) -> Result<Vec<Book>, Error>;
    async fn update(&self, book: Book) -> Result<Book, Error>;
    async fn delete(&self, id: i64) -> Result<(), Error>;
}

