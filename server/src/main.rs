mod controller;
mod db;
mod middleware;
mod models;
mod repository;
mod routes;
mod service;

use actix_web::{App, HttpServer, web};
use db::init_db;
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool: PgPool = init_db().await.expect("Failed to initialize the database.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
