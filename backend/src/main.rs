mod struct_entity;
mod repositories;
mod db;


use std::error::Error;
use std::env;
use dotenv::dotenv;
use crate::db::db_config::DbInitialization;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL Failed");

     let db_init = DbInitialization::new(
        db_url.into(),
    );

    let pool = db_init.db_connect().await?;
    db_init.db_migration(&pool).await?;



    
    Ok(())
}
