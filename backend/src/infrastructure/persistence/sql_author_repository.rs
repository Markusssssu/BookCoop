use async_trait::async_trait;
use sqlx::{PgPool, FromRow};
use crate::application::repositories::author_repository::AuthorRepository;
use crate::domain::author::{Author, NewAuthor};

pub struct SQLAuthorRepository {
    pub pool: PgPool,
}

#[async_trait]
impl AuthorRepository for SQLAuthorRepository {
    async fn insert(&self, author: NewAuthor) -> Result<Author, anyhow::Error> {
        let row = sqlx::query_as::<_, Author>(
            r#"
            INSERT INTO Authors (full_name, date_of_birth, biography)
            VALUES ($1, $2, $3)
            RETURNING author_id, full_name, date_of_birth, biography
            "#
        )
            .bind(author.full_name)
            .bind(author.date_of_birth)
            .bind(author.biography)
            .fetch_one(&self.pool)
            .await?;

        Ok(row)
    }

    async fn find_all(&self) -> Result<Vec<Author>, anyhow::Error> {
        let rows = sqlx::query_as::<_, Author>(
            "SELECT author_id, full_name, date_of_birth, biography FROM Authors"
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Author>, anyhow::Error> {
        let row = sqlx::query_as::<_, Author>(
            "SELECT author_id, full_name, date_of_birth, biography FROM Authors WHERE author_id = $1"
        )
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row)
    }

    async fn find_by_full_name(&self, name: &str) -> Result<Vec<Author>, anyhow::Error> {
        let rows = sqlx::query_as::<_, Author>(
            "SELECT author_id, full_name, date_of_birth, biography FROM Authors WHERE full_name ILIKE $1"
        )
            .bind(format!("%{}%", name))
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }

    async fn update(&self, id: i32, author: NewAuthor) -> Result<Author, anyhow::Error> {
        let row = sqlx::query_as::<_, Author>(
            r#"
            UPDATE Authors
            SET full_name = $1, date_of_birth = $2, biography = $3
            WHERE author_id = $4
            RETURNING author_id, full_name, date_of_birth, biography
            "#
        )
            .bind(author.full_name)
            .bind(author.date_of_birth)
            .bind(author.biography)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row)
    }

    async fn delete(&self, id: i32) -> Result<(), anyhow::Error> {
        sqlx::query("DELETE FROM Authors WHERE author_id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
