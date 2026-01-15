use async_trait::async_trait;
use sqlx::{PgPool, Row, FromRow};
use crate::application::repositories::book_issues_repository::BookIssuesRepository;
use crate::domain::book_issues::{BookIssue, NewBookIssue};

pub struct SQLBookIssuesRepository {
    pool: PgPool,
}

#[async_trait]
impl BookIssuesRepository for SQLBookIssuesRepository {
    async fn insert(&self, book: NewBookIssue) -> Result<BookIssue, anyhow::Error> {
        let row = sqlx::query_as::<_, BookIssue>(
            r#"
            INSERT INTO Book_Issues (book_id, author_id, issue_date, return_date)
            VALUES ($1, $2, $3, $4)
            RETURNING issue_id, book_id, author_id, issue_date, return_date
            "#
        )
            .bind(book.book_id)
            .bind(book.author_id)
            .bind(book.issue_date)
            .bind(book.return_date)
            .fetch_one(&self.pool)
            .await?;

        Ok(row)
    }

    async fn find_all(&self) -> Result<Vec<BookIssue>, anyhow::Error> {
        let rows = sqlx::query_as::<_, BookIssue>(
            "SELECT issue_id, book_id, author_id, issue_date, return_date FROM Book_Issues"
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<BookIssue>, anyhow::Error> {
        let row = sqlx::query_as::<_, BookIssue>(
            "SELECT issue_id, book_id, author_id, issue_date, return_date FROM Book_Issues WHERE issue_id = $1"
        )
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row)
    }

    async fn find_by_title(&self, title: &str) -> Result<Vec<BookIssue>, anyhow::Error> {
        let rows = sqlx::query_as::<_, BookIssue>(
            r#"
            SELECT bi.issue_id, bi.book_id, bi.author_id, bi.issue_date, bi.return_date
            FROM Book_Issues bi
            JOIN Books b ON bi.book_id = b.book_id
            WHERE b.title ILIKE $1
            "#
        )
            .bind(format!("%{}%", title))
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }

    async fn update(&self, id: i32, book: NewBookIssue) -> Result<BookIssue, anyhow::Error> {
        let row = sqlx::query_as::<_, BookIssue>(
            r#"
            UPDATE Book_Issues
            SET book_id = $1, author_id = $2, issue_date = $3, return_date = $4
            WHERE issue_id = $5
            RETURNING issue_id, book_id, author_id, issue_date, return_date
            "#
        )
            .bind(book.book_id)
            .bind(book.author_id)
            .bind(book.issue_date)
            .bind(book.return_date)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row)
    }

    async fn delete(&self, id: i32) -> Result<(), anyhow::Error> {
        sqlx::query("DELETE FROM Book_Issues WHERE issue_id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
