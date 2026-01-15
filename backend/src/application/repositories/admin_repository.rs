use async_trait::async_trait;
use crate::domain::admin::{Admin, NewAdmin};

#[async_trait]
pub trait AdminRepository: Send + Sync {
    /*=======Base CRUD=======*/
    async fn insert(&self, admin: NewAdmin) -> Result<Admin, anyhow::Error>;
    async fn find_all(&self) -> Result<Vec<Admin>, anyhow::Error>;
    async fn find_by_id(&self, id: i32) -> Result<Option<Admin>, anyhow::Error>;
    async fn find_by_full_name(&self, full_name: &str) -> Result<Vec<Admin>, anyhow::Error>;
    async fn update(&self, id: i32, admin: NewAdmin) -> Result<Admin, anyhow::Error>;
    async fn delete(&self, id: i32) -> Result<(), anyhow::Error>;

    /*=======================*/
}
