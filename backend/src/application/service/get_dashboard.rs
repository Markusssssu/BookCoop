pub async fn get_dashboard(
    State(db): State<PgPool>
) -> Json<Dashboard> {

    let books = sqlx::query_scalar!("SELECT COUNT(*) FROM Books")
        .fetch_one(&db)
        .await
        .unwrap();

    let authors = sqlx::query_scalar!("SELECT COUNT(*) FROM Authors")
        .fetch_one(&db)
        .await
        .unwrap();

    let today = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM Book_Issues WHERE issue_date = CURRENT_DATE"
    )
    .fetch_one(&db)
    .await
    .unwrap();

    let cpu = 0.42; 

    Json(Dashboard { books, authors, today, cpu })
}