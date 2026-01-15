use async_trait::async_trait;
use sqlx::PgPool;
use crate::domain::book::{Book, NewBook};
use crate::application::repositories::book_repository::BookRepository;
use anyhow::Result;

pub struct SQLBookRepository {
    pub pool: PgPool,
}

#[async_trait]
impl BookRepository for SQLBookRepository {
    async fn insert(&self, book: NewBook) -> Result<Book> {
        let rec = sqlx::query_as::<_, Book>(
            r#"
            INSERT INTO books ()
            VALUES ()
            RETURNING
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
            r#"SELECT id, title, author, published_year FROM books"#
        )
            .fetch_all(&self.pool)
            .await?;
        Ok(books)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Book>> {
        let book = sqlx::query_as::<_, Book>(
            r#"SELECT FROM books WHERE id = $1"#
        )
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(book)
    }

    async fn find_by_title(&self, title: &str) -> Result<Vec<Book>> {
        let books = sqlx::query_as::<_, Book>(
            r#"SELECT FROM books WHERE title ILIKE $1"#
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
            SET title = $1, author = $2, published_year = $3
            WHERE id = $4
            RETURNING id, title, author, published_year
            "#
        )
            .bind(book.title)
            .bind(book.author)
            .bind(book.published_year)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(updated)
    }

    async fn delete(&self, id: i32) -> Result<()> {
        sqlx::query(
            r#"DELETE FROM books WHERE id = $1"#
        )
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
