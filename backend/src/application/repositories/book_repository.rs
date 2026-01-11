pub trait BookRepository {
    async fn insert(&self, book: Book) -> Result<Book, RepoError>;
    async fn get(&self, id: i64) -> Result<Option<Book>, RepoError>;
    async fn list(&self) -> Result<Vec<Book>, RepoError>;
    async fn update(&self, book: Book) -> Result<Book, RepoError>;
    async fn delete(&self, id: i64) -> Result<(), RepoError>;
}