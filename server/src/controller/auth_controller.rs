use crate::service::auth_service;
use actix_web::cookie::Cookie;
use actix_web::{HttpResponse, Responder, post, web};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct AuthRequest {
    email: String,
    password: String,
}

#[post("/register")]
async fn register(pool: web::Data<PgPool>, data: web::Json<AuthRequest>) -> impl Responder {
    match auth_service::register(pool.get_ref(), &data.email, &data.password).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/login")]
async fn login(pool: web::Data<PgPool>, data: web::Json<AuthRequest>) -> impl Responder {
    auth_service::login(pool.get_ref(), &data.email, &data.password)
        .await
        .map_or(HttpResponse::Unauthorized().finish(), |t| {
            HttpResponse::Ok()
                .cookie(
                    Cookie::build("auth_token", t)
                        .http_only(true)
                        .secure(true)
                        .path("/")
                        .finish(),
                )
                .finish()
        })
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(register).service(login));
}
