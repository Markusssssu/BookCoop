use std::sync::Arc;
use crate::domain::book::{Book, NewBook};
use crate::application::repositories::book_repository::BookRepository;

pub struct BookService {
    repository: Arc<dyn BookRepository + Send + Sync>,
}

impl BookService {
    pub fn new(repository: Arc<dyn BookRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn create_book(&self, book_data: NewBook) -> Result<Book, anyhow::Error> {
        if book_data.title.trim().is_empty() {
            return Err(anyhow::anyhow!("Название книги не может быть пустым"));
        }
        self.repository.insert(book_data).await
    }

    pub async fn get_all_books(&self) -> Result<Vec<Book>, anyhow::Error> {
        self.repository.find_all().await
    }

    pub async fn get_book_by_id(&self, id: i32) -> Result<Book, anyhow::Error> {
        self.repository.find_by_id(id).await?
            .ok_or_else(|| anyhow::anyhow!("Книга с ID {} не найдена", id))
    }

    pub async fn update_book(&self, id: i32, data: NewBook) -> Result<Book, anyhow::Error> {
        self.get_book_by_id(id).await?;
        self.repository.update(id, data).await
    }

    pub async fn delete_book(&self, id: i32) -> Result<(), anyhow::Error> {
        self.repository.delete(id).await
    }
}
