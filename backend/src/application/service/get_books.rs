pub async fn get_books(
    State(db): State<PgPool>
) -> Json<Vec<Book>> {
    let books = sqlx::query_as!(
        Book,
        r#"
        SELECT book_id, title, author, genre, page_count
        FROM Books
        "#
    )
    .fetch_all(&db)
    .await
    .unwrap();

    Json(books)
}
