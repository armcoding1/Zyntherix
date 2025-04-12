use crate::models::user::CreateUser;
use crate::service::user_service;
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use sqlx::PgPool;

#[post("/users/create")]
pub async fn create_user(pool: web::Data<PgPool>, data: web::Json<CreateUser>) -> impl Responder {
    match user_service::create_user(pool.get_ref(), &data.email, &data.password_hash).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/users/find_all")]
pub async fn find_all(pool: web::Data<PgPool>) -> impl Responder {
    match user_service::find_all(pool.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/users/find_by_id/{id}")]
pub async fn find_by_id(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    match user_service::find_by_id(pool.get_ref(), id.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[delete("/users/delete_by_id/{id}")]
pub async fn delete_by_id(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    match user_service::delete_by_id(pool.get_ref(), id.into_inner()).await {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[put("/users/update_by_id/{id}")]
pub async fn update_by_id(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    data: web::Json<CreateUser>,
) -> impl Responder {
    match user_service::update_by_id(
        pool.get_ref(),
        id.into_inner(),
        Some(&data.email),
        Some(&data.password_hash),
    )
    .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user);
    cfg.service(find_all);
    cfg.service(find_by_id);
    cfg.service(delete_by_id);
    cfg.service(update_by_id);
}
