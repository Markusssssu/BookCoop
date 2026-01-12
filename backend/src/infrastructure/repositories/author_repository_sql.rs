/*=====use====== */
use sqlx::{Pool, Postgres};
/*============== */

/*=======use crate=======*/
use crate::application::repositories::author_repository::AuthorRepository;
use crate::domain::author::Author;
use std::io::Error;

/*=======================*/

struct AuhtorRepositorySql {
    pool: Pool<Postgres>,
}