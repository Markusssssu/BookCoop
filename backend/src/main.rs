/*========mod========*/
mod application;
mod domain;
mod infrastructure;
mod presentation;
/*================== */

/*========use crate::========*/
use crate::infrastructure::server::handler::Handler;
use crate::infrastructure::db::connection::DBConnection;
use crate::infrastructure::log::logger_init::tracing_initialization;
/*================== */

/*========use========*/
use std::error::Error;
/*================== */

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let log_init = tracing_initialization().await;
    let db_conn = DBConnection::new().await;
    let handler = Handler::new().await;
    handler.run().await;

    Ok(()) 
}
   