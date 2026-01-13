pub async fn get_authors(
    State(db): State<PgPool>
) -> Json<Vec<Author>> {
    let authors = sqlx::query_as!(
        Author,
        r#"
        SELECT author_id, full_name, date_of_birth::text, biography
        FROM Authors
        "#
    )
    .fetch_all(&db)
    .await
    .unwrap();

    Json(authors)
}
