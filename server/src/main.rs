mod controller;
mod db;
mod models;
mod repository;
mod routes;
mod service;

use actix_web::{App, HttpServer, web};
use db::init_db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = init_db().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
