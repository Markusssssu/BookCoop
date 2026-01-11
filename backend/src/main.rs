/*========mod========*/
mod application;
mod domain;
mod infrastructure;
mod presentation;
/*================== */

/*========use crate::========*/
use crate::infrastructure::db::connection::DBConnection;
use crate::infrastructure::log::logger_init::tracing_initialization;
use crate::infrastructure::server::handler::Handler;
/*================== */

/*========use========*/
use std::error::Error;
/*================== */

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let handler = Handler::new().await;
    handler.run().await;

    Ok(()) 
}
   