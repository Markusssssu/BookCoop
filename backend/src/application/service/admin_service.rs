use std::sync::Arc;
use crate::domain::admin::{Admin, NewAdmin};
use crate::application::repositories::admin_repository::AdminRepository;

pub struct AdminService {
    repository: Arc<dyn AdminRepository + Send + Sync>,
}

impl AdminService {
    pub fn new(repository: Arc<dyn AdminRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn list_admins(&self) -> Result<Vec<Admin>, anyhow::Error> {
        self.repository.find_all().await
    }

    pub async fn get_admin_by_id(&self, id: i32) -> Result<Admin, anyhow::Error> {
        self.repository.find_by_id(id).await?
            .ok_or_else(|| anyhow::anyhow!("Администратор с ID {} не найден", id))
    }

    pub async fn update_admin(&self, id: i32, data: NewAdmin) -> Result<Admin, anyhow::Error> {
        // Сначала проверяем, существует ли он
        self.get_admin_by_id(id).await?;

        self.repository.update(id, data).await
    }

    pub async fn delete_admin(&self, id: i32) -> Result<(), anyhow::Error> {
        // Можно добавить логику, чтобы нельзя было удалить "последнего" админа
        let all = self.repository.find_all().await?;
        if all.len() <= 1 {
            return Err(anyhow::anyhow!("Нельзя удалить последнего администратора в системе"));
        }

        self.repository.delete(id).await
    }

    pub async fn search_by_name(&self, name: &str) -> Result<Vec<Admin>, anyhow::Error> {
        self.repository.find_by_full_name(name).await
    }
}
