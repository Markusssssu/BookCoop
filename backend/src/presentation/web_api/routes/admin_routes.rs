// // routes/admin_routes.rs
// use axum::{Router, routing::{get, post, put, delete}, extract::Extension};
// use std::sync::Arc;
// use crate::application::service::admin_service::AdminService;
// use crate::presentation::web_api::handler::admin_handler::*;
// 
// pub fn admin_routes(service: Arc<AdminService>) -> Router {
//     Router::new()
//         .route("/admins", post(create_admin).get(get_all_admins))
//         .route("/admins/:id",
//                get(get_admin_by_id)
//                    .put(update_admin)
//                    .delete(delete_admin)
//         )
//         .layer(Extension(service))
// }
