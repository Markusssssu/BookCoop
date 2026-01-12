/*=====use====== */
use sqlx::{Pool, Postgres};
/*============== */

/*=======use crate=======*/
use crate::application::repositories::book_repository::BookRepository;
use crate::domain::book::Book;
use std::io::Error;

/*=======================*/

struct BookRepositorySql {
    pool: Pool<Postgres>,
}

// impl BookRepository for BookRepositorySql {

//     async fn insert(&self, book: Book) -> Result<Book, Error> {

//     }

//     async fn get(&self, id: i64) -> Result<Option<Book>, Error > {

//     }

//     async fn list(&self) -> Result<Vec<Book>, Error> {

//     }

//     async fn update(&self, book: Book) -> Result<Book, Error> {

//     }

//     async fn delete(&self, id: i64) -> Result<(), Error> {

//     }
// }

