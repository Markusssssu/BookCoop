use std::sync::Arc;
use crate::domain::author::{Author, NewAuthor};
use crate::application::repositories::author_repository::AuthorRepository;

pub struct AuthorService {
    repository: Arc<dyn AuthorRepository + Send + Sync>,
}

impl AuthorService {
    pub fn new(repository: Arc<dyn AuthorRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn create_author(&self, author_data: NewAuthor) -> Result<Author, anyhow::Error> {
   
        if author_data.full_name.trim().is_empty() {
            return Err(anyhow::anyhow!("Имя автора не может быть пустым"));
        }

        self.repository.insert(author_data).await
    }

    pub async fn get_all_authors(&self) -> Result<Vec<Author>, anyhow::Error> {
        self.repository.find_all().await
    }

    pub async fn get_author_by_id(&self, id: i32) -> Result<Author, anyhow::Error> {
        let author = self.repository.find_by_id(id).await?;

        author.ok_or_else(|| anyhow::anyhow!("Автор с ID {} не найден", id))
    }

    pub async fn search_authors_by_name(&self, name: &str) -> Result<Vec<Author>, anyhow::Error> {
        if name.len() < 2 {
            return Err(anyhow::anyhow!("Поисковый запрос слишком короткий"));
        }

        self.repository.find_by_full_name(name).await
    }

    pub async fn update_author(&self, id: i32, author_data: NewAuthor) -> Result<Author, anyhow::Error> {
      
        self.get_author_by_id(id).await?;

        self.repository.update(id, author_data).await
    }

    pub async fn delete_author(&self, id: i32) -> Result<(), anyhow::Error> {
        

        self.repository.delete(id).await
    }
}
