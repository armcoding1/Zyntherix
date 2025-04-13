// use crate::middleware::auth_middleware::AuthMiddleware;
// use crate::models::user::CreateUser;
// use crate::service::user_service;
// use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
// use sqlx::PgPool;

// pub fn config(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::scope("/users")
//             .wrap(AuthMiddleware)
//             .service(create_user)
//             .service(find_all)
//             .service(find_by_id)
//             .service(delete_by_id)
//             .service(update_by_id),
//     );
// }
