use std::sync::Arc;
use crate::domain::book_issues::{BookIssue, NewBookIssue};
use crate::application::repositories::book_issues_repository::BookIssuesRepository;
use crate::application::repositories::book_repository::BookRepository;
use crate::application::repositories::author_repository::AuthorRepository;

pub struct BookIssueService {
    issue_repo: Arc<dyn BookIssuesRepository + Send + Sync>,
    book_repo: Arc<dyn BookRepository + Send + Sync>,
    author_repo: Arc<dyn AuthorRepository + Send + Sync>,
}

impl BookIssueService {
    pub fn new(
        issue_repo: Arc<dyn BookIssuesRepository + Send + Sync>,
        book_repo: Arc<dyn BookRepository + Send + Sync>,
        author_repo: Arc<dyn AuthorRepository + Send + Sync>,
    ) -> Self {
        Self { issue_repo, book_repo, author_repo }
    }
    pub async fn create_issue(&self, data: NewBookIssue) -> Result<BookIssue, anyhow::Error> {
        let book = self.book_repo.find_by_id(data.book_id).await?;
        if book.is_none() {
            return Err(anyhow::anyhow!("Книга с ID {} не существует", data.book_id));
        }

        let author = self.author_repo.find_by_id(data.author_id).await?;
        if author.is_none() {
            return Err(anyhow::anyhow!("Автор с ID {} не существует", data.author_id));
        }

        self.issue_repo.insert(data).await
    }

    pub async fn get_all_issues(&self) -> Result<Vec<BookIssue>, anyhow::Error> {
        self.issue_repo.find_all().await
    }

    pub async fn get_issue_by_id(&self, id: i32) -> Result<BookIssue, anyhow::Error> {
        self.issue_repo.find_by_id(id).await?
            .ok_or_else(|| anyhow::anyhow!("Запись о выдаче №{} не найдена", id))
    }

    pub async fn update_issue(&self, id: i32, data: NewBookIssue) -> Result<BookIssue, anyhow::Error> {
        self.issue_repo.update(id, data).await
    }

    pub async fn delete_issue(&self, id: i32) -> Result<(), anyhow::Error> {
        self.issue_repo.delete(id).await
    }
}

