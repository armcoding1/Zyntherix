use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::user::User;
use crate::service::user_service;

#[post("/users")]
async fn add_user(pool: web::Data<PgPool>, data: web::Json<User>) -> impl Responder {
    let user: User = user_service::create_new_user(pool.get_ref(), &data.email, &data.password_hash).await;
    HttpResponse::Created().json(user)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(add_user);
}