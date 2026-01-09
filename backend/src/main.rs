mod struct_entity;
mod repositories;

use std::error::Error;
use std::env;
use dotenv::dotenv;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL Failed");
    let pool = sqlx::PgPool::connect(&url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(())
}
