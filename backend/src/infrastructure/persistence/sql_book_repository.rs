use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Result;

use crate::domain::book::{Book, NewBook};
use crate::application::repositories::book_repository::BookRepository;

pub struct SQLBookRepository {
    pub pool: PgPool,
}

#[async_trait]
impl BookRepository for SQLBookRepository {

    async fn insert(&self, book: NewBook) -> Result<Book> {
        let rec = sqlx::query_as::<_, Book>(
            r#"
            INSERT INTO books (title, author, genre, page_count)
            VALUES ($1, $2, $3, $4)
            RETURNING book_id, title, author, genre, page_count
            "#
        )
            .bind(book.title)
            .bind(book.author)
            .bind(book.genre)
            .bind(book.page_count)
            .fetch_one(&self.pool)
            .await?;

        Ok(rec)
    }

    async fn find_all(&self) -> Result<Vec<Book>> {
        let books = sqlx::query_as::<_, Book>(
            r#"
            SELECT book_id, title, author, genre, page_count
            FROM books
            ORDER BY book_id
            "#
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(books)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Book>> {
        let book = sqlx::query_as::<_, Book>(
            r#"
            SELECT book_id, title, author, genre, page_count
            FROM books
            WHERE book_id = $1
            "#
        )
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(book)
    }

    async fn find_by_title(&self, title: &str) -> Result<Vec<Book>> {
        let books = sqlx::query_as::<_, Book>(
            r#"
            SELECT book_id, title, author, genre, page_count
            FROM books
            WHERE title ILIKE $1
            "#
        )
            .bind(format!("%{}%", title))
            .fetch_all(&self.pool)
            .await?;

        Ok(books)
    }

    async fn update(&self, id: i32, book: NewBook) -> Result<Book> {
        let updated = sqlx::query_as::<_, Book>(
            r#"
            UPDATE books
            SET title = $1,
                author = $2,
                genre = $3,
                page_count = $4
            WHERE book_id = $5
            RETURNING book_id, title, author, genre, page_count
            "#
        )
            .bind(book.title)
            .bind(book.author)
            .bind(book.genre)
            .bind(book.page_count)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(updated)
    }

    async fn delete(&self, id: i32) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM books
            WHERE book_id = $1
            "#
        )
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
