use async_trait::async_trait;
use sqlx::{PgPool, FromRow};
use crate::application::repositories::admin_repository::AdminRepository;
use crate::domain::admin::{Admin, NewAdmin};

pub struct SQLAdminRepository {
    pub pool: PgPool,
}

#[async_trait]
impl AdminRepository for SQLAdminRepository {
    async fn insert(&self, admin: NewAdmin) -> Result<Admin, anyhow::Error> {
        let row = sqlx::query_as::<_, Admin>(
            r#"
            INSERT INTO Admin (full_name, keyword)
            VALUES ($1, $2)
            "#
        )
            .bind(admin.)
            .bind(admin.)
            .fetch_one(&self.pool)
            .await?;

        Ok(row)
    }

    async fn find_all(&self) -> Result<Vec<Admin>, anyhow::Error> {
        let rows = sqlx::query_as::<_, Admin>(
            "SELECT admin_id, full_name, keyword FROM Admin"
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Admin>, anyhow::Error> {
        let row = sqlx::query_as::<_, Admin>(
            "SELECT admin_id, full_name, keyword FROM Admin WHERE admin_id = $1"
        )
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row)
    }

    async fn find_by_full_name(&self, full_name: &str) -> Result<Vec<Admin>, anyhow::Error> {
        let rows = sqlx::query_as::<_, Admin>(
            "SELECT admin_id, full_name, keyword FROM Admin WHERE full_name ILIKE $1"
        )
            .bind(format!("%{}%", full_name))
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }

    async fn update(&self, id: i32, admin: NewAdmin) -> Result<Admin, anyhow::Error> {
        let row = sqlx::query_as::<_, Admin>(
            r#"
            UPDATE Admin
            SET full_name = $1, keyword = $2
            WHERE admin_id = $3
            RETURNING admin_id, full_name, keyword
            "#
        )
            .bind(admin.full_name)
            .bind(admin.keyword)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row)
    }

    async fn delete(&self, id: i32) -> Result<(), anyhow::Error> {
        sqlx::query("DELETE FROM Admin WHERE admin_id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
